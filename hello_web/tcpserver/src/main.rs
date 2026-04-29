use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    //将TcpListener绑定到本地IP和端口
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("运行在3000端口上");

    //只接收一次请求
    //let result = listener.accept().unwrap();
    
    //使用incoming方法持续接收请求
    /*for stream in listener.incoming(){
        let _stream = stream.unwrap();
        println!("建立联系！！！{:?}", _stream);
    }*/

    //使用incoming方法持续接收请求并接收消息
    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        println!("建立联系！！！");
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
