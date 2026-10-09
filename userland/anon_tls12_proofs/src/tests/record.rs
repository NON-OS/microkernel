// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Recording a handshake from a real OpenSSL server, on demand only:
//! vectors/record.py starts `openssl s_server` for each case and runs
//!
//!   NONOS_TLS12_RECORD=<name>:<port> cargo test --release record_openssl -- --ignored
//!
//! which writes vectors/<name>.server, <name>.client and <name>.outcome.

use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::vec::Vec;

use nonos_tls::{Io, SessionError};

use super::replay::run;

struct Recorder {
    tcp: TcpStream,
    server: Vec<u8>,
    client: Vec<u8>,
}

impl Io for Recorder {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.client.extend_from_slice(data);
        self.tcp.write_all(data).map_err(|_| SessionError::Io)
    }
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        match self.tcp.read(into) {
            Ok(0) => Ok(0),
            Ok(n) => {
                self.server.extend_from_slice(&into[..n]);
                Ok(n)
            }
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => Ok(0),
            Err(_) => Err(SessionError::Io),
        }
    }
}

#[test]
#[ignore = "needs an openssl s_server started by vectors/record.py"]
fn record_openssl() {
    let spec = std::env::var("NONOS_TLS12_RECORD").expect("name:port");
    let (name, port) = spec.split_once(':').expect("name:port");
    let tcp = TcpStream::connect(("127.0.0.1", port.parse::<u16>().unwrap())).expect("s_server listening");
    tcp.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    let mut rec = Recorder { tcp, server: Vec::new(), client: Vec::new() };
    let began = Instant::now();
    let outcome = run(&mut rec, || began.elapsed() > Duration::from_secs(6));
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/vectors/");
    std::fs::write(std::format!("{dir}{name}.server"), &rec.server).unwrap();
    std::fs::write(std::format!("{dir}{name}.client"), &rec.client).unwrap();
    std::fs::write(std::format!("{dir}{name}.outcome"), outcome.line() + "\n").unwrap();
    std::println!("{name}: {}", outcome.line());
}
