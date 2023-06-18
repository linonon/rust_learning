// 使用 std::io::prelude::*; 导入了一些 IO trait，
// 使我们可以使用类似于 write_all 和 copy 这样的方法，
// 而无需显式导入它们。
use std::io::prelude::*;
// 同时，也导入了一些预定义的类型，
// 例如 std::net::TcpStream，这个类型代表了一个 TCP 连接。
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut connection = TcpStream::connect("www.rustinaction.com:80")?; // We need to specify the port (80) explicitly,
                                                                         // TcpStream does not know that this will become a
                                                                         // HTTP request.
                                                                         // 在 main 函数中，我们尝试连接到一个主机名为 "www.rustinaction.com"
                                                                         // 的 HTTP 服务器的端口为 80 的 TCP 端口。我们需要明确指定端口 (80)，
                                                                         // 因为 TcpStream 并不知道这将成为一个 HTTP 请求。

    connection.write_all(b"GET / HTTP/1.0\r")?; // GET is the HTTP method, / is the resource we're
                                                // attempting to access and HTTP/1.0 is the protocol
                                                // version we're requesting. Why 1.0? It does not
                                                // support "keep alive" requests, which will allow
                                                // our stream to close without difficulty.
                                                // 然后，我们将 HTTP 请求的方法、请求资源以及协议版本通过 write_all 方法写入了连接中。
                                                // 具体来说，我们使用 GET 作为 HTTP 方法，/ 作为我们尝试访问的资源，
                                                // HTTP/1.0 作为请求的协议版本。为什么使用 1.0 版本？
                                                // 因为它不支持“保持连接”请求，这样我们的流就可以轻松关闭。
    connection.write_all(b"\r\n")?;
    connection.write_all(b"Host: www.rustinaction.com")?; // The hostname provided on line 5 is actually
                                                          // discarded once it is converted to an IP address.
                                                          // The Host HTTP header allows the server to know
                                                          // which host we're connecting to..
                                                          // 接下来，我们写入了一个空行和 Host HTTP 标头，
                                                          // 以告诉服务器我们连接的主机名。事实上，第 5 行提供的主机名实际上被丢弃了，
                                                          // 一旦它被转换为 IP 地址，服务器就不会再使用它了。
    connection.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut connection, &mut std::io::stdout())?;
    Ok(())
}
