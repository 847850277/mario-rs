use bytes::Bytes;
use http_body::Body as HttpBody;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tokio::runtime::Runtime;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    //req.into_parts();
    let mut body = req.into_body();
    let mut data = Vec::new();

    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        data.extend_from_slice(&chunk);
    }

    let body_string = String::from_utf8_lossy(&data);
    println!("Received body: {}", body_string);

    Ok(Response::new(Body::from("Received")))
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let make_svc =
            make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle_request)) });

        let addr = ([127, 0, 0, 1], 8080).into();
        let server = Server::bind(&addr).serve(make_svc);

        println!("Listening on http://{}", addr);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    });
}
