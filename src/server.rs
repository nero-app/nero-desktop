use axum::Router;
use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    router: Router,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            router: Router::new(),
        }
    }

    pub fn extend(mut self, router: Router) -> Self {
        self.router = self.router.merge(router);
        self
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let address = self.listener.local_addr()?;

        tracing::info!(%address, "the app server is listening");

        axum::serve(self.listener, self.router).await
    }
}
