use crate::types::{HealthCheck, HealthCheckKind};
use anyhow::{bail, Context, Result};
use std::net::{SocketAddr, TcpStream};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

fn attempt(hc: &HealthCheck) -> Result<()> {
    match hc.kind {
        HealthCheckKind::None => Ok(()),
        HealthCheckKind::Http => {
            let url = hc.url.clone().unwrap_or_default();
            let res = ureq::get(&url)
                .timeout(Duration::from_millis(hc.timeout_ms.unwrap_or(5000)))
                .call();
            match res {
                Ok(r) => {
                    let s = r.status();
                    if (200..400).contains(&s) {
                        Ok(())
                    } else {
                        bail!("http {}", s)
                    }
                }
                Err(e) => bail!("http err {}", e),
            }
        }
        HealthCheckKind::Tcp => {
            let port = hc.tcp_port.unwrap_or(0);
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            if TcpStream::connect_timeout(
                &addr,
                Duration::from_millis(hc.timeout_ms.unwrap_or(2000)),
            )
            .is_ok()
            {
                Ok(())
            } else {
                bail!("tcp")
            }
        }
        HealthCheckKind::Command => {
            let cmd = hc.command.clone().unwrap_or_default();
            if cmd.is_empty() {
                bail!("empty command")
            }
            let status = if cfg!(windows) {
                Command::new("cmd")
                    .arg("/C")
                    .arg(cmd)
                    .status()
                    .context("command")?
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .status()
                    .context("command")?
            };
            if status.success() {
                Ok(())
            } else {
                bail!("command failed")
            }
        }
    }
}

pub fn wait_ready(hc: &HealthCheck) -> Result<()> {
    let retries = hc.retries.unwrap_or(10);
    let timeout_ms = hc.timeout_ms.unwrap_or(5000);
    let start = Instant::now();
    for _ in 0..retries {
        let r = attempt(hc);
        if r.is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(300));
        if start.elapsed() > Duration::from_millis(timeout_ms * 2) {
            break;
        }
    }
    bail!("not ready")
}
