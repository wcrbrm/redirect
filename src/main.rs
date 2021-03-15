use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let domain = std::env::var("REDIRECT").unwrap();
    let out = format!("{}{}", domain, req.uri());
    Ok(Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header(header::LOCATION, out)
        .body(Body::from(""))
        .unwrap())
}

#[tokio::main]
async fn main() {
    let domain = std::env::var("REDIRECT").unwrap_or("".to_owned());
    if domain.len() == 0 {
        println!("REDIRECT variable must be specified");
        std::process::exit(-1);
    }
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("Starting server at 0.0.0.0:3000, redirecting to {}", domain);
    // let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler)) });
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler)) });

    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
