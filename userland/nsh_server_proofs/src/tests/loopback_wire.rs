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

//! The client side of the loopback, and what the wire shows. The hellos
//! travel in plaintext, so their suite and key sizes are read straight off
//! the bytes: an X-Wing encapsulation key is ML-KEM-768's 1184 bytes then
//! X25519's 32, and its ciphertext ML-KEM-768's 1088 then X25519's 32. The
//! record the host signed is checked half by half: Ed25519 and ML-DSA-65
//! each verify alone, and a record with either half spoiled is refused.

use super::loopback_dial::dial;
pub use super::loopback_got::{Got, Loop};
use super::loopback_host::urandom;
use crate::kit::roles_build::grant::GrantB;
use nsh_client::Event as C;
use std::io::{ErrorKind, Read, Write};
use std::net::SocketAddr;
use std::time::{Duration, Instant};

/// Connect, log in with `grant`, type each of `lines` once logged in, and
/// run until the session ends.
pub fn client(addr: SocketAddr, grant: &GrantB, lines: &[&[u8]]) -> Got {
    let (mut c, mut sock) = dial(addr, grant);
    let mut got = Got { err: None, events: Vec::new(), c2s: Vec::new(), s2c: Vec::new(), th: None };
    let (mut typed, mut buf) = (false, vec![0u8; 65_536]);
    let deadline = Instant::now() + Duration::from_secs(30);
    while !c.is_over() && Instant::now() < deadline {
        let out = c.take_output();
        got.c2s.extend_from_slice(&out);
        sock.write_all(&out).expect("write");
        match sock.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                got.s2c.extend_from_slice(&buf[..n]);
                if let Err(e) = c.feed(&buf[..n], &mut urandom) {
                    got.err = Some(e);
                }
            }
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(e) => panic!("read: {e}"),
        }
        let new = c.events();
        if !typed && new.iter().any(|e| matches!(e, C::LoggedIn(_))) {
            lines.iter().try_for_each(|l| c.input(l)).expect("input");
            typed = true;
        }
        got.events.extend(new);
    }
    let out = c.take_output();
    got.c2s.extend_from_slice(&out);
    // The host may already have closed the connection after its CLOSE.
    let _ = sock.write_all(&out);
    got.th = c.th_server_hello();
    got
}
