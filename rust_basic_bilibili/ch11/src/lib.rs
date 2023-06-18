#[derive(Debug)]
pub struct Rectangle {
    l: u32,
    w: u32,
}

impl Rectangle {
    pub fn new(l: u32, w: u32) -> Rectangle {
        Rectangle { w, l }
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.l > other.l && self.w > other.w
    }
}

/*
    測試默認多線程
    --test-threads=1 單線程測試

    默認 測試通過時, println! 相關的輸出會被捕獲
    cargo test -- --show-outpu 打印成功時的 println!

    cargo test fn_name 可以測試單個函數

    cargo test mod_name 可測試模塊

    cargo test fn_name_slice 可測試包含 fn_name_slice 的測試

    #[cfg(test)] 單元測試
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "reason unknown"]
    fn experation() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "gg")]
    fn another() {
        panic!("gg")
    }

    #[test]
    fn rectangle_test() {
        let r1 = Rectangle::new(8, 7);
        let r2 = Rectangle::new(9, 2);

        assert!(
            r1.can_hold(&r2),
            "\nRectangle cannot hold another: \nsrc: {:?}\nother: {:?}\n",
            r1,
            r2
        );
    }
}
