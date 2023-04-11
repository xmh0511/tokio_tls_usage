use tokio::{net::TcpStream,time::{self,Duration}, io::{AsyncWriteExt, AsyncReadExt,BufStream}};

use native_tls::TlsConnector;
use async_socks5::{connect, Result};
#[tokio::main]
async fn main() ->Result<(),tokio::io::Error> { 
    //let r = TcpStream::connect("1.117.226.170:6000").await?;
    // let conn = time::timeout(Duration::from_secs(5), TcpStream::connect("www.google.com:443")).await??;
    // let cx = TlsConnector::builder().build().unwrap();
    // let cx = tokio_native_tls::TlsConnector::from(cx);
    // let mut socket = cx.connect("blog.xmh0511.top", conn).await.unwrap();
    // let r = socket.write(b"GET / HTTP/1.1\r\n\r\n").await?;
    // println!("{r:?}");
    // let mut s = String::new();
    // let r = socket.read_to_string(& mut s).await?;
    // println!("{r:?}, {s}");
    let mut stream = TcpStream::connect("127.0.0.1:7891").await?;
    //let mut stream = BufStream::new(stream);
    let r = connect(&mut stream, ("www.baidu.com", 80), None).await.unwrap();
    println!("{r:?}");
    let cx = TlsConnector::builder().build().unwrap();
    let cx = tokio_native_tls::TlsConnector::from(cx);
    let mut socket = cx.connect("www.baidu.com", stream).await.unwrap();
    let r = socket.write_all(b"GET / HTTP/1.1\r\n\r\n").await?;
    println!("{r:?}");
    let mut s = String::new();
    let mut bff = [0u8;128];
    let size = socket.read(& mut bff).await?;
    println!("{size}, {:?}",std::str::from_utf8(&bff));
    Ok(())
}
