use std::sync::Arc;

use krustie::{
    http::{core::Context, listener::Listener},
    krustie::{request::Request, response::Response, router::Router},
};

struct EmptyContext;

impl Context for EmptyContext {}

#[tokio::test]
async fn krustie() {
    let router = Router::new();
    let context = EmptyContext {};
    let listener =
        Listener::<Request, Response, EmptyContext, Router<Request, Response, EmptyContext>>::new(
            router,
            Arc::new(context),
            256 as usize,
        );

    match listener.listen(8080).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
