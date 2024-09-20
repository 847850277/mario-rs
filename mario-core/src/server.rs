use tokio::net::TcpListener;

pub struct Server {

}

impl Server {

    pub fn new() -> Server {
        Server{
        }
    }

    pub(crate) async fn start_server(&self) {
        //tokio web server bind port
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        println!("Server running on {}", listener.local_addr().unwrap());
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            // let router = Arc::clone(&self.router); // Clone the router to use in the spawned task
            // tokio::spawn(async move {
            //     handle_connection(stream,router).await;
            // });
        }
    }

}

