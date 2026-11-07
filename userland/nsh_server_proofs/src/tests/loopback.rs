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

//! The server over TCP on 127.0.0.1, with `/bin/sh` behind it, and the real
//! client against it: an attested host is accepted and a command runs; a
//! host whose kernel cannot attest is refused before any credential leaves;
//! a user the role table does not allow is refused. The bytes on the wire
//! show the hybrid suite: X-Wing (X25519 with ML-KEM-768) in the hellos,
//! and Ed25519 with ML-DSA-65 on the record the host signs. Randomness is
//! the system's; the host clock is the role table's, moving as real time
//! does.

use super::loopback_host::serve;
use super::loopback_suite::hellos;
use super::loopback_wire::client;
use crate::kit::host::Kernel;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use nsh_client::Event as C;
use nsh_server::{EndReason, Event as S};
use std::net::TcpListener;
use std::thread;

#[test]
fn an_attested_host_runs_a_command_and_the_suite_is_hybrid() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let host = thread::spawn(move || serve(&listener, Kernel::honest(), Table::shipped()));
    let got = client(addr, &GrantB::default(), &[b"echo nsh-loopback-$((6*7))\n", b"exit\n"]);
    let served = host.join().expect("the host thread");
    assert_eq!(got.err, None, "{:?}", got.events);
    let out: Vec<u8> = got
        .events
        .iter()
        .filter_map(|e| match e {
            C::Output(b) => Some(b.clone()),
            _ => None,
        })
        .flatten()
        .collect();
    assert!(
        String::from_utf8_lossy(&out).contains("nsh-loopback-42"),
        "{}",
        String::from_utf8_lossy(&out)
    );
    assert!(got.events.contains(&C::Exit(0)) && got.events.ends_with(&[C::Closed]));
    assert!(served.contains(&S::Closed(EndReason::Exit)));
    hellos(&got.c2s, &got.s2c);
    super::loopback_suite::record_halves(&got);
}
