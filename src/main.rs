use crate::common_ports::MOST_COMMON_PORTS_100;
use rayon::prelude::*;

mod common_ports;

use std::net::{TcpStream, ToSocketAddrs};
use std::{net::SocketAddr, time::Duration};

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter() // <- HERE IS THE IMPORTANT BIT
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open) // filter closed ports
        .collect();
    subdomain
}

fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);

    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    Port {
        port: port,
        is_open,
    }
}
fn main() {
    let server_details = "stackoverflow.com:80";
    let mut addrs_iter = server_details.to_socket_addrs().unwrap();
    scan_port(addrs_iter.next().unwrap(), 443);
    let mut subdomains = Vec::new();
    let port = Port {
        port: 443,
        is_open: false,
    };
    let google = Subdomain {
        domain: "google.com".to_string(),
        open_ports: vec![port],
    };
    subdomains.push(google);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    // pool.install is required to use our custom threadpool, instead of rayon's default one
    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains
            //.unwrap()
            .into_par_iter()
            .map(scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}", port.port);
            }

            println!();
        }
    });
}

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}
