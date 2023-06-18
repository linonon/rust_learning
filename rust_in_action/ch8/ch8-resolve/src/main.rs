use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use clap::{App, Arg};
use trust_dns::{
    op::{Message, MessageType, OpCode, Query},
    rr::Name,
    serialize::binary::*,
};

fn main() {
    let app = App::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(
            Arg::with_name("dns-server")
                .short("s")
                .default_value("1.1.1.1"),
        )
        .arg(Arg::with_name("domain").required(true))
        .get_matches();

    // 1. domain_name_raw 和 domain_name 分别表示
    // 从命令行参数(arg)中获取的原始字符串和转换为 trust_dns::rr::Name 类型的域名。
    let domain_name_raw = app.value_of("domain").unwrap();
    let domain_name = Name::from_ascii(domain_name_raw).unwrap();

    // 2. dns_server_raw 和 dns_server 分别表示
    // 从命令行参数(arg)中获取的原始字符串和转换为 std::net::SocketAddr 类型的 DNS 服务器地址。
    let dns_server_raw = app.value_of("dns-server").unwrap(); // 2.
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw) //2.
        .parse()
        .expect("invalid address");

    // 3. request_as_bytes 和 response_as_bytes 分别是请求和响应的字节数组。
    // request_as_bytes 通过 Vec::with_capacity 方法预先分配了 512 字节的内存空间，
    // 而 response_as_bytes 通过 vec! 宏直接初始化了 512 字节的空间并填充了 0。
    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512); // 3.
    let mut response_as_bytes: Vec<u8> = vec![0; 512]; // 3.

    let mut msg = Message::new(); // 4. msg 是一个 trust_dns::op::Message 类型的对象，表示 DNS 协议中的一个消息。

    msg.set_id(rand::random::<u16>()) // 5.
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, trust_dns::rr::RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    // 6. encoder 是一个 trust_dns::serialize::binary::BinEncoder 类型的对象，
    // 用于将消息序列化成字节数组。emit 方法将 msg 序列化并写入 request_as_bytes。
    let mut encoder = BinEncoder::new(&mut request_as_bytes); // 6.
    msg.emit(&mut encoder).unwrap(); // 6.

    // 7. localhost 是一个 std::net::UdpSocket 类型的对象，表示本地的 UDP 套接字。
    let localhost = UdpSocket::bind("0.0.0.0:0").expect("can't bind to address");

    let timeout = Duration::from_secs(3); // 8. timeout 表示超时时间。
    localhost.set_read_timeout(Some(timeout)).unwrap(); // 8. 设置超时时间。
    localhost.set_nonblocking(false).unwrap(); // 9. 设置为阻塞模式。

    let _amt = localhost
        .send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured"); // 10. 发送请求。
    let (_amt, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("timeout reached"); // 11. 接收响应。

    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");
    // 12. dns_message 是一个 trust_dns::op::Message 类型的对象，表示 DNS 协议中的一个消息。
    // Message::from_vec 方法将 response_as_bytes 字节数组解析成 trust_dns::op::Message 类型的对象，
    // 并存储在 dns_message 中。dns_message.answers() 方法返回 DNS 响应中的答案部分，
    // 并遍历每个答案，如果答案的类型是 IPv4 地址（A 记录），则将其打印出来。

    for answer in dns_message.answers() {
        if answer.record_type() == trust_dns::rr::RecordType::A {
            let resource = answer.rdata();
            let ip = resource.to_ip_addr().expect("invalid IP address");
            println!("{}", ip.to_string());
        }
    }
}
