use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    //建立连接
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    //向服务端发送消息
    stream.write("Hello".as_bytes()).unwrap();

    //接收服务端返回的消息
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    //打印消息
    println!(
        "服务器响应：{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
