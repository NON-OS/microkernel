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


use super::answering::Answering;
use crate::stream::connect_unauthenticated;

/// Records fixtures/openssl_tls12_alert.tls, on demand only:
///   openssl s_server -accept 44400 -tls1_2 -cert c.pem -key k.pem &
///   NONOS_TLS12_SERVER=127.0.0.1:44400 cargo test --release record_openssl_tls12 -- --ignored
#[test]
#[ignore = "needs openssl s_server -tls1_2 listening"]
fn record_openssl_tls12() {
    use std::io::{Read, Write};
    let addr = std::env::var("NONOS_TLS12_SERVER").expect("ip:port");
    let mut tcp = std::net::TcpStream::connect(addr).expect("s_server");
    tcp.set_read_timeout(Some(std::time::Duration::from_secs(3))).unwrap();
    let mut io = Answering { answer: |_: &[u8]| Vec::new(), hello: None, queued: Vec::new() };
    let _ = connect_unauthenticated(&mut io, b"relay.example");
    tcp.write_all(io.hello.as_ref().unwrap()).unwrap();
    let mut answer = std::vec![0u8; 4096];
    let n = tcp.read(&mut answer).unwrap();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/fixtures/openssl_tls12_alert.tls");
    std::fs::write(path, &answer[..n]).unwrap();
}
