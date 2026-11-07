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
use super::loopback_wire::client;
use crate::kit::host::Kernel;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use nsh_client::{Event as C, HostRefusal, Refusal};
use nsh_server::{AttestFault, Event as S};
use nsh_wire::Reason;
use std::net::TcpListener;
use std::thread;

#[test]
fn a_host_whose_kernel_cannot_attest_is_refused_before_any_credential() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let kernel = Kernel { fault: Some(AttestFault::Kernel(-1)), ..Kernel::honest() };
    let host = thread::spawn(move || serve(&listener, kernel, Table::shipped()));
    let got = client(addr, &GrantB::default(), &[]);
    let served = host.join().expect("the host thread");
    assert_eq!(got.err, Some(Refusal::Aborted(Reason::Busy)));
    assert!(!got.events.contains(&C::HostAccepted));
    assert!(served.iter().all(|e| !matches!(e, S::Admit(_))));
}

#[test]
fn a_host_attesting_another_handshake_is_refused_by_the_client() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let kernel = Kernel { stale: true, ..Kernel::honest() };
    let host = thread::spawn(move || serve(&listener, kernel, Table::shipped()));
    let got = client(addr, &GrantB::default(), &[]);
    host.join().expect("the host thread");
    let refused = matches!(got.err, Some(Refusal::Host(HostRefusal::Attest(_))));
    assert!(refused, "{:?}", got.err);
    assert!(!got.events.contains(&C::HostAccepted));
}
