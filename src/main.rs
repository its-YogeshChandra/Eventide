//create http wrapper of our own
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use std::convert::Infallible;
use std::future;
use tokio::net::TcpListener;

// work would be done in layers

//bind tcp listener to the address
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listner = tokio::spawn(TcpListener::bind(address));
    let val = listner.await?;
    println!("the server is like : {:#?}", val.unwrap());
    Ok(())
}

async fn hello(_req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("hello world "))))
}
