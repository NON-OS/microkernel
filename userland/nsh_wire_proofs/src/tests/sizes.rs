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

//! The caps hold the largest bodies the format allows: the largest legal
//! `HOST_PROOF` and `USER_AUTH` fit the handshake cap, the largest
//! `ESCALATE` fits the session cap, and each passes `split` there.

use crate::build::enrol::EnrolB;
use crate::build::grant::GrantB;
use crate::build::proof::{escalate, user_auth, HostProofB};
use nsh_wire::{split, Expect, HostProof, Kind, UserAuth};
use nsh_wire::{ENTRY_LEN, MAX_DOC, MAX_ENTRIES, MAX_GRANT};

fn framed(kind: Kind, body: &[u8]) -> Vec<u8> {
    [&[kind.byte()][..], body].concat()
}

pub fn largest_grant() -> GrantB {
    GrantB {
        user: vec![b'u'; 32],
        role: vec![b'r'; 16],
        caps: (0..nsh_wire::MAX_CAPS).map(|i| format!("C{i:031}").into_bytes()).collect(),
        approvers: vec![[0xA1; 32], [0xA2; 32]],
        ..GrantB::escalation()
    }
}

#[test]
fn the_largest_grant_is_max_grant() {
    let g = largest_grant().bytes();
    assert_eq!(g.len(), MAX_GRANT);
    assert!(nsh_wire::Grant::parse(&g).is_ok());
}

#[test]
fn the_largest_host_proof_fits_the_handshake_cap() {
    let host = [&[b'a'; 63][..], b".", &[b'b'; 63], b".", &[b'c'; 63], b".", &[b'd'; 61]];
    let b = HostProofB {
        enrol: EnrolB { host: host.concat(), ..EnrolB::default() }.bytes(),
        doc: vec![0xD0; MAX_DOC],
        entries: vec![0; ENTRY_LEN * MAX_ENTRIES],
    };
    let body = b.bytes();
    assert!(HostProof::parse(&body).is_ok());
    let inner = framed(Kind::HostProof, &body);
    assert!(split(&inner, Expect::HostProof).is_ok(), "{} bytes", inner.len());
}

#[test]
fn the_largest_user_auth_fits_the_handshake_cap() {
    let base = GrantB { approvers: vec![], kind: 1, parent_id: [0; 16], ..largest_grant() };
    let body = user_auth(&base.bytes());
    assert!(UserAuth::parse(&body).is_ok());
    assert!(split(&framed(Kind::UserAuth, &body), Expect::UserAuth).is_ok());
}

#[test]
fn the_largest_escalate_fits_the_session_cap() {
    let body = escalate(&largest_grant().bytes());
    assert!(nsh_wire::escalate(&body).is_ok());
    assert!(split(&framed(Kind::Escalate, &body), Expect::ServerSession).is_ok());
}
