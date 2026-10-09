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

//! The real client against live Anyone relays over TCP, on demand only:
//! `cargo test --release live_relays -- --ignored --nocapture`. Relays answer
//! ECDHE over P-256 alone, so this is the handshake the link layer depends on.
//! NONOS_LIVE_RELAYS overrides the list as comma separated ip:port.

use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::session::{Io, SessionError};

struct Tcp(TcpStream);

impl Io for Tcp {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.0.write_all(data).map_err(|_| SessionError::Io)
    }
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        match self.0.read(into) {
            Ok(0) => Err(SessionError::Io),
            Ok(n) => Ok(n),
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => Ok(0),
            Err(_) => Err(SessionError::Io),
        }
    }
}

const RELAYS: &str = "212.132.126.60:9001,109.230.255.78:9001,38.242.206.66:9001";

#[test]
#[ignore = "network: dials live Anyone relays"]
fn live_relays_complete_the_handshake() {
    let list = std::env::var("NONOS_LIVE_RELAYS").unwrap_or_else(|_| RELAYS.into());
    for relay in list.split(',') {
        let addr = relay.parse().expect("ip:port");
        let tcp = TcpStream::connect_timeout(&addr, Duration::from_secs(10)).expect("tcp");
        tcp.set_read_timeout(Some(Duration::from_millis(200))).expect("timeout");
        let got = crate::stream::connect_unauthenticated(&mut Tcp(tcp), b"www.nonos-link.net");
        println!("{relay}: {:?}", got.as_ref().map(|s| s.is_done()));
        assert!(got.is_ok(), "{relay}: {:?}", got.err());
    }
}
