use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

pub struct Scanner {
    sem: Arc<Semaphore>,
    timeout: Duration,
}

impl Scanner {
    pub fn new(concurrency: usize, timeout_ms: u64) -> Self {
        Self {
            sem: Arc::new(Semaphore::new(concurrency)),
            timeout: Duration::from_millis(timeout_ms),
        }
    }

    /// Single host: try each port, return the first open one.
    pub async fn probe(&self, ip: IpAddr, ports: &[u16]) -> Option<SocketAddr> {
        for &p in ports {
            let _permit = self.sem.acquire().await.ok()?;
            let addr = SocketAddr::new(ip, p);
            if let Ok(Ok(_)) = timeout(self.timeout, TcpStream::connect(addr)).await {
                return Some(addr);
            }
        }
        None
    }
}
