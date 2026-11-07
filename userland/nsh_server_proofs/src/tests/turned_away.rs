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

//! A host that serves no one tells the real client why, over TCP, before
//! any key exists: a machine with no TPM answers `UNATTESTED`, one that
//! holds no enrolment or role table `UNENROLLED`, as nsh.front does
//! (session.rs). The client reads the plaintext frame, sends nothing more,
//! and `nsh` says the sentence and exits 255; no other reason may be told
//! this way.

use super::cli::nsh;
use super::cli_files::files;
use super::loopback_wire::client;
use crate::kit::roles_build::grant::GrantB;
use nsh_client::connect::explain::refusal;
use nsh_client::{Event as C, Refusal};
use nsh_server::turned_away;
use nsh_wire::Reason;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

/// A host that reads the client's hello and answers `reason` in plaintext.
fn turning_away(reason: Reason) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let frame = turned_away(reason).expect("a reason told before keys");
    let host = thread::spawn(move || {
        let (mut sock, _) = listener.accept().expect("a connection");
        let mut hello = [0u8; 4096];
        let _ = sock.read(&mut hello);
        sock.write_all(&frame).expect("the answer");
    });
    (addr, host)
}

#[test]
fn the_client_hears_why_a_host_serves_no_one() {
    for reason in [Reason::Unattested, Reason::Unenrolled, Reason::Busy] {
        let (addr, host) = turning_away(reason);
        let got = client(addr.parse().expect("addr"), &GrantB::default(), &[]);
        host.join().expect("the host");
        assert_eq!(got.err, Some(Refusal::Aborted(reason)));
        assert!(!got.events.contains(&C::HostAccepted));
    }
}

#[test]
fn nsh_exits_255_and_says_why() {
    let (addr, host) = turning_away(Reason::Unenrolled);
    let (_, args) = files("unenrolled", &addr, &GrantB::default());
    let (status, out) = nsh(&args, &[]);
    host.join().expect("the host");
    assert_eq!(status, 255, "{out}");
    assert!(out.is_empty(), "no session ran: {out}");
    let said = refusal(&Refusal::Aborted(Reason::Unenrolled));
    assert!(said.starts_with("the host turned the connection away: "), "{said}");
    assert!(said.contains("nsh-enrol"), "{said}");
}

#[test]
fn only_a_reason_nsh_lets_a_host_tell_before_keys_is_framed() {
    for code in 0..=u16::MAX {
        let Ok(r) = Reason::from_code(code) else { continue };
        let allowed =
            matches!(r, Reason::Version | Reason::Busy | Reason::Unattested | Reason::Unenrolled);
        assert_eq!(turned_away(r).is_some(), allowed, "{r:?}");
    }
}
