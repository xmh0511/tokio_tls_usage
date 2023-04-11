use tokio::{net::TcpStream,time::{self,Duration}, io::{AsyncWriteExt, AsyncReadExt,BufStream}};

use native_tls::TlsConnector;
use async_socks5::{ Result};

async fn tls_over_socks5() ->Result<(),tokio::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:7891").await?;
    let r = async_socks5::connect(&mut stream, ("google.com", 443), None).await.unwrap();
    println!("{r:?}");
    let cx = TlsConnector::builder().build().unwrap();
    let cx = tokio_native_tls::TlsConnector::from(cx);
    let mut socket = cx.connect("google.com", stream).await.unwrap();
    let r = socket.write_all(b"GET / HTTP/1.1\r\n\r\n").await?;
    println!("{r:?}");
    let mut buffer = [0u8;256];
    let r = socket.read(& mut buffer).await?;
    println!("{r}, {:?}",std::str::from_utf8(&buffer));
    Ok(())
}
async fn tls_connection()->Result<(),tokio::io::Error>{
    let conn = time::timeout(Duration::from_secs(5), TcpStream::connect("blog.xmh0511.top:443")).await??;
    let cx = TlsConnector::builder().build().unwrap();
    let cx = tokio_native_tls::TlsConnector::from(cx);
    let mut socket = cx.connect("blog.xmh0511.top", conn).await.unwrap();
    let r = socket.write(b"GET / HTTP/1.1\r\n\r\n").await?;
    println!("{r:?}");
    let mut s = String::new();
    let r = socket.read_to_string(& mut s).await?;
    println!("{r:?}, {s}");
    Ok(())
}
#[tokio::main]
async fn main() ->Result<(),tokio::io::Error> { 
    tls_connection().await?;
    println!("-------------------------");
    tls_over_socks5().await?;
    Ok(())
}
