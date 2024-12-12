extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use noli::net::{lookup_host, SocketAddr, TcpStream};
use saba_core::{error::Error, http::HttpResponse};

pub struct HttpClient {}
impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    /// HTTP GET
    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    e
                )))
            }
        };

        if ips.is_empty() {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();

        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ));
            }
        };

        let request_line = format!("GET /{} HTTP/1.1\n", &path);

        let header = [
            format!("Host: {}\n", &host),
            "Accept: text/html\n".to_string(),
            "Connection: close\n".to_string(),
            "\n".to_string(),
        ]
        .join("");

        let request = [request_line, header].join("");

        // リクエストの送信
        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to send a request to TCP stream".to_string(),
                ));
            }
        };

        // レスポンスの受信
        let mut recieved = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network(
                        "Failed to receive a request from TCP stream".to_string(),
                    ));
                }
            };
            if bytes_read == 0 {
                break;
            }
            recieved.extend_from_slice(&buf[..bytes_read]);
        }

        // HTTPレスポンスの構築
        match core::str::from_utf8(&recieved) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(e) => Err(Error::Network(format!("Invalid received response: {}", e))),
        }
    }
}
