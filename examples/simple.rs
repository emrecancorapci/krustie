
#[tokio::main]
async fn main() {
    let mut server = krustie::Server::create();
    let mut router = krustie::Router::new();

    router.get("/", |_, res| {
        res.status(krustie::StatusCode::Ok).body_text("Hello World!");
    });

    server.use_handler(router);

    server.listen(8080).await;
}