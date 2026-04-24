//create http wrapper of our own
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use hyper_util::server::conn::auto;
use std::convert::Infallible;
use tokio::{net::TcpListener, select};

// work would be done in layers

//bind tcp listener to the address
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";

    //ech connnection is in task
    let listener = tokio::spawn(TcpListener::bind(address));
    for (_, stream) in listener.await?.iter().enumerate() {
        println!("the stream is : {:#?}", stream);

        //wrap the stream in the hyper tokio io adapter
    }

    //shutdown the connection
    Ok(())
}

async fn hello(_req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("hello world "))))
}
