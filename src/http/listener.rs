use std::{marker::PhantomData, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};

use super::core::{Context, HttpRequest, HttpResponse, RouteHandling};

#[derive(Debug)]
pub struct Listener<Req, Res, Ctx, RH>
where
    Req: HttpRequest,
    Res: HttpResponse,
    Ctx: Context + 'static,
    RH: RouteHandling<Req, Res, Ctx> + 'static,
{
    router: Arc<RH>,
    context: Arc<Ctx>,
    max_queue_size: usize,
    _req: PhantomData<Req>,
    _res: PhantomData<Res>,
}

impl<Req, Res, Ctx, RH> Listener<Req, Res, Ctx, RH>
where
    Req: HttpRequest,
    Res: HttpResponse,
    Ctx: Context,
    RH: RouteHandling<Req, Res, Ctx> + 'static,
{
    pub fn new(router: RH, context: Arc<Ctx>, max_queue_size: usize) -> Self {
        Listener {
            router: Arc::new(router),
            context,
            max_queue_size,
            _req: PhantomData,
            _res: PhantomData,
        }
    }

    pub async fn listen(&self, port: u16) -> std::io::Result<()> {
        let address = format!("0.0.0.0:{}", port);
        let listener = Arc::new(TcpListener::bind(&address).await?);

        println!("Listening on http://{}", address);

        let (tx, mut rx) = mpsc::channel(self.max_queue_size);

        for _ in 0..self.max_queue_size {
            while let Some(stream) = rx.recv().await {
                let router = self.router.clone();
                let context = self.context.clone();

                tokio::spawn(async move {
                    if let Err(e) = Self::handle_stream(stream, router, context).await {
                        eprintln!("Error handling request: {}", e);
                    }
                });
            }
        }

        while let Ok((stream, _)) = listener.accept().await {
            if tx.send(stream).await.is_err() {
                eprintln!("Request queue is full! Dropping connection.");
            }
        }

        Ok(())
    }

    async fn handle_stream(
        mut stream: TcpStream,
        router: Arc<RH>,
        context: Arc<Ctx>,
    ) -> std::io::Result<()> {
        let mut buffer = vec![0; 4096]; // Increased buffer size
        let bytes_read = stream.read(&mut buffer).await?;
        buffer.truncate(bytes_read);

        let peer_addr = stream.peer_addr()?;
        let mut request = Req::new(&buffer, peer_addr).await?;

        let response_bytes: Vec<u8> = router
            .handle(&mut request, HttpResponse::new(), context)
            .await
            .into();

        stream.write_all(&response_bytes).await?;
        Ok(())
    }
}
