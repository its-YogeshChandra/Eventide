//create http wrapper of our own
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{Request, Response, server};
use hyper_util::rt::TokioExecutor;
use hyper_util::{rt::TokioIo, server::conn::auto};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::{net::TcpListener, select};
use tokio_util::sync::CancellationToken;

//bind tcp listener to the address
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    //ech connnection is in task
    let listener = TcpListener::bind(address).await?;

    loop {
        let (stream, _socket_addr) = listener.accept().await?;
        println!("the socket address is : {:#?}", _socket_addr);
        println!("the stream is : {:#?}", stream);

        //wrap the stream in the hyper tokio io ada
        let io = TokioIo::new(stream);
        println!("the io is : {:#?}", io);

        let mut is_error = false;

        //spawn tokio for mutliple connection concurrently
        tokio::task::spawn(async move {
           let conn  =  auto::Builder::new(TokioExecutor::new())
                .serve_connection(io, service_fn(dispatcher))
                .await;
            tokio::pin!(conn);
            let child_token = CancellationToken::new();

            loop {
                tokio::select! {
                    result = &mut conn => {
                        break;
                    }
                    _ = child_token.
                }
            }
            {
                is_error = true;
                eprintln!("Error serving connection: {}", e)
            }
            if is_error == true {
                return;
            }
        });
    }

    //shutdown the connection
    Ok(())
}

async fn dispatcher(_req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("hello world "))))
}