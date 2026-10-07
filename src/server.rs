// -----------------------------------------------------------------------------
// Public server API
// -----------------------------------------------------------------------------

use crate::conn::handle_conn;
use crate::{Error, Result};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;

pub struct Server {
    addr: SocketAddr,
    idle_timeout: Duration,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            idle_timeout: Duration::from_secs(60),
        }
    }

    pub fn idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    pub async fn run(self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)
            .await
            .map_err(|source| Error::Bind {
                addr: self.addr,
                source,
            })?;

        loop {
            let (stream, peer) = listener.accept().await.map_err(Error::Accept)?;
            let idle_timeout = self.idle_timeout;

            tokio::spawn(async move {
                if let Err(err) = handle_conn(stream, idle_timeout).await {
                    eprintln!("connection {peer} closed: {err}");
                }
            });
        }
    }
}
