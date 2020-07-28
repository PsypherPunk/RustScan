use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use colored::*;
use futures::stream::FuturesUnordered;
use std::time::Duration;
use std::{
    io::ErrorKind,
    net::{Shutdown, SocketAddr, IpAddr, Ipv6Addr, Ipv4Addr},
};

pub struct Scanner {
    host: IpAddr,
    start: u64,
    end: u64,
    batch_size: u64,
    timeout: Duration,
    quiet: bool,
    ipv6: bool,
}

impl Scanner {
    pub fn new(
        host: IpAddr,
        start: u64,
        end: u64,
        batch_size: u64,
        timeout: Duration,
        quiet: bool,
        ipv6: bool,
    ) -> Self {
        Self {
            host: host.to_owned(),
            start,
            end,
            batch_size,
            timeout,
            quiet,
            ipv6,
        }
    }

    pub async fn run(&self) -> Vec<u64> {
        let ports: Vec<u64> = (self.start..self.end).collect();
        let mut open_ports: std::vec::Vec<u64> = Vec::new();

        for range in ports.chunks(self.batch_size as usize) {
            let mut ports = self.scan_range(range).await;
            open_ports.append(&mut ports);
        }

        open_ports
    }

    async fn scan_range(&self, range: &[u64]) -> Vec<u64> {
        let mut ftrs = FuturesUnordered::new();

        for port in range {
            ftrs.push(self.scan_port(port));
        }

        let mut open_ports: Vec<u64> = Vec::new();
        while let Some(result) = ftrs.next().await {
            match result {
                Ok(port) => open_ports.push(port),
                _ => {}
            }
        }

        open_ports
    }

    async fn scan_port(&self, port: u16) -> SocketAddr {
        let addr = SocketAddr::new(self.host, 80);


        return addr;
        
    }

    async fn connect(&self, addr: SocketAddr) -> io::Result<TcpStream> {
        let stream =
            io::timeout(self.timeout, async move { TcpStream::connect(addr).await }).await?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_it_run() {
        // TODO run functions here
        assert_eq!(1, 1);
    }
}