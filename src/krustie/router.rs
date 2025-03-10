use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::http::core::{Context, HttpRequest, HttpResponse, RouteHandling, Routing};

pub struct Router<Req: HttpRequest, Res: HttpResponse, Ctx> {
    middlewares: Vec<Box<dyn RouteHandling<Req, Res, Ctx>>>,
    routes: Vec<Box<dyn RouteHandling<Req, Res, Ctx>>>,
}

impl<Req, Res, Ctx> Router<Req, Res, Ctx>
where
    Req: HttpRequest,
    Res: HttpResponse,
{
    pub fn new() -> Self {
        todo!()
    }
}

impl<Req, Res, Ctx> Routing<Req, Res, Ctx> for Router<Req, Res, Ctx>
where
    Req: HttpRequest,
    Res: HttpResponse,
    Ctx: Context,
{
    fn get_params(&self, path: &str) -> HashMap<String, String> {
        todo!()
    }
}

#[async_trait]
impl<Req, Res, Ctx> RouteHandling<Req, Res, Ctx> for Router<Req, Res, Ctx>
where
    Req: HttpRequest,
    Res: HttpResponse,
    Ctx: Context,
{
    async fn handle(&self, request: &mut Req, response: Res, context: Arc<Ctx>) -> Res {
        todo!()
    }

    fn use_router(&self, path: String, router: Self) {
        todo!()
    }
}
