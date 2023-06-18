use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
use serde::{Deserialize, Serialize};

type ByteString = Vec<u8>;

type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f, index })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            self.index.insert(kv.key, position);
        }

        Ok(())
    }

    fn process_record<R: io::Read>(
        f: &mut R, //1. f可以是任何实现读取的类型，比如读取文件的类型，但也可以是&[u8]。
    ) -> io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<byteorder::LittleEndian>()?; // 2.  如下一节所述，byteorder crate允许以确定性的方式读取磁盘上的整数。
        let key_len = f.read_u32::<byteorder::LittleEndian>()?; // 2
        let val_len = f.read_u32::<byteorder::LittleEndian>()?; // 2
        let data_len = key_len + val_len; // 2

        let mut data = ByteString::with_capacity(data_len as usize);
        {
            f.by_ref() // 3. f.by_ref()是必须的，因为take(n)创建了一个新的Read值。在这个短暂的区块中使用一个引用，可以避免所有权问题。
                .take(data_len as u64)
                .read_to_end(&mut data)?;
        }
        debug_assert_eq!(data.len(), data_len as usize); // 4. debug_assert! tests are disabled in optimized builds, enabling debug builds to have more runtime checks.
                                                         // 4. 要注意的是，debug_assert_eq! 在优化构建时会被禁用，只有在调试构建中才会进行运行时检查。

        let checksum = crc32::checksum_ieee(&data); // 5. 使用 crc32::checksum_ieee 方法计算读取的数据的校验和。
                                                    //    校验和是一个数字，用于验证从磁盘读取的数据与预期的数据是否相同。
                                                    //    本段代码中使用 IEEE 802.3 标准中规定的 CRC-32 算法来计算数据的校验和。
                                                    //    具体实现可以参考第 7.7.4 节。
        if checksum != saved_checksum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }

        let value = data.split_off(key_len as usize); // 6. The split_off(n) method splits a Vec<T> in two at n.
                                                      // 6. split_off(n) 方法将 data 切分为两个部分，前面 n 个字节被切分成一个
                                                      //    新的 ByteString 类型的值 value，而后面的字节则被保留为 key。
                                                      //    最后，使用 Ok(KeyValuePair { key, value }) 包装一个 KeyValuePair 类型的结果返回。
        let key = data;

        return Ok(KeyValuePair { key, value });
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        self.f.seek(SeekFrom::End(0))
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            Some(position) => *position,
            None => return Ok(None),
        };

        let kv = self.get_at(position)?;

        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;

        let kv = ActionKV::process_record(&mut f)?;

        Ok(kv)
    }

    /// 在文件中查找一个键，并返回它的位置和值。
    /// - `target`：要查找的键
    ///
    /// 返回值：
    /// - 如果找到了键，则返回它的位置（单位为字节）和值
    /// - 如果没有找到，则返回 `None`
    /// - 如果发生错误，则返回一个 `std::io::Error` 类型的错误
    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);

        let mut found: Option<(u64, ByteString)> = None; // 初始化找到的结果为空

        loop {
            let position = f.seek(SeekFrom::Current(0))?; // 获取当前位置

            let maybe_kv = ActionKV::process_record(&mut f); // 处理文件中的一个键值对
            let kv = match maybe_kv {
                Ok(kv) => kv, // 如果处理成功，则返回键值对
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => break, // 如果文件读到末尾，则退出循环
                    _ => return Err(err),                  // 否则返回错误
                },
            };

            if kv.key == target {
                // 如果找到了目标键
                found = Some((position, kv.value)); // 更新找到的结果
            }

            // important to keep looping until the end of the file,
            // incase the key has been overwritten
            // 重要提示：为了防止键被覆盖，需要一直循环到文件末尾
        }

        return Ok(found); // 返回找到的结果
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;

        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    /// 向文件中插入一个键值对，但不考虑索引，返回插入的位置。
    /// - `key`：要插入的键
    /// - `value`：要插入的值
    ///
    /// 返回值：
    /// - 成功时，返回插入的位置（单位为字节）
    /// - 失败时，返回一个 `std::io::Error` 类型的错误
    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len(); // 计算键的长度
        let val_len = value.len(); // 计算值的长度
        let mut tmp = ByteString::with_capacity(key_len + val_len); // 申请一个容量为键长+值长的 ByteString

        // 将键和值按顺序压入 tmp 中
        for byte in key {
            tmp.push(*byte);
        }
        for byte in value {
            tmp.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp); // 计算 tmp 的校验和

        let next_byte = SeekFrom::End(0); // 计算下一个字节的位置（即文件末尾）
        let current_position = f.seek(SeekFrom::Current(0))?; // 获取当前位置
        f.seek(next_byte)?; // 将文件指针移动到文件末尾
        f.write_u32::<LittleEndian>(checksum)?; // 将校验和写入文件（使用小端字节序）
        f.write_u32::<LittleEndian>(key_len as u32)?; // 将键的长度写入文件（使用小端字节序）
        f.write_u32::<LittleEndian>(val_len as u32)?; // 将值的长度写入文件（使用小端字节序）
        f.write_all(&tmp)?; // 将键值对写入文件

        Ok(current_position) // 返回插入的位置
    }

    #[inline] // 告诉编译器将这些函数内联到调用它们的函数中
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline] // 告诉编译器将这些函数内联到调用它们的函数中
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }
}
