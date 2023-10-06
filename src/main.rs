use std::net::{Ipv4Addr, UdpSocket};

extern crate clap;
use clap::Parser;
#[derive(Parser)]
struct Args {
    #[arg(env)]
    server_address: String,
    #[arg(env)]
    server_id: String,
    #[arg(env)]
    server_name: String,
    endpoint_address: Option<String>,
}

extern crate dotenvy;

#[allow(clippy::unwrap_used)]
fn main() -> std::io::Result<()> {
    // try to load env file
    let _unused = dotenvy::dotenv();

    let Args {
        server_address,
        server_id,
        server_name,
        endpoint_address,
    } = Args::parse();

    let sockv4 = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 7359))?;

    let mut rx_buf: [u8; 128] = [0; 128];

    let endpoint_string = match endpoint_address {
        Some(a) => format!(r#""{a}""#),
        None => "null".to_owned(),
    };

    let response = format!(
        r#"{{"Address":"{server_address}","Id":"{server_id}","Name":"{server_name}","EndpointAddress":{endpoint_string}}}"#
    );

    loop {
        match sockv4.recv_from(&mut rx_buf) {
            Ok((len, remote)) => {
                if let Ok(message) = std::str::from_utf8_mut(&mut rx_buf[..len]) {
                    if message == "Who is JellyfinServer?" {
                        println!("sending jellyfin discovery message");

                        sockv4.send_to(response.as_bytes(), remote).unwrap();
                    }
                }
            }
            Err(_) => continue,
        }
    }
}
