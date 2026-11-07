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
//! `HOST_PROOF` fits the handshake cap, the largest `USER_AUTH` and
//! `AUTH_OK` fill their stages' caps exactly, the largest `ESCALATE` fits the
//! session cap, and each passes `split` there. One byte more is refused
//! before it is read.

use crate::build::enrol::EnrolB;
use crate::build::grant::GrantB;
use crate::build::proof::{escalate, user_auth, HostProofB};
use nsh_wire::{split, wire_len, AuthOk, Expect, HostProof, Kind, UserAuth, WireError};
use nsh_wire::{ENTRY_LEN, MAX_AUTH_OK, MAX_BASE_GRANT, MAX_DOC, MAX_ENTRIES, MAX_GRANT};
use nsh_wire::{MAX_USER_AUTH, TAG_LEN};

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

fn largest_base() -> GrantB {
    GrantB { approvers: vec![], kind: 1, parent_id: [0; 16], ..largest_grant() }
}

/// The stage refuses `inner` and its length field one byte over.
fn one_more_is_refused(inner: &[u8], e: Expect) {
    let mut over = inner.to_vec();
    over.push(0);
    assert_eq!(split(&over, e), Err(WireError::Oversize));
    let sealed = u32::try_from(over.len() + TAG_LEN).expect("a length");
    assert_eq!(wire_len(sealed.to_le_bytes(), e), Err(WireError::Oversize));
}

#[test]
fn the_largest_base_grant_is_max_base_grant() {
    let g = largest_base().bytes();
    assert_eq!(g.len(), MAX_BASE_GRANT);
    assert!(nsh_wire::Grant::parse(&g).is_ok());
}

#[test]
fn the_largest_user_auth_fills_its_cap() {
    let body = user_auth(&largest_base().bytes());
    assert_eq!(body.len(), MAX_USER_AUTH);
    assert!(UserAuth::parse(&body).is_ok());
    let inner = framed(Kind::UserAuth, &body);
    assert_eq!(inner.len(), Expect::UserAuth.cap());
    assert!(split(&inner, Expect::UserAuth).is_ok());
    one_more_is_refused(&inner, Expect::UserAuth);
}

#[test]
fn the_largest_auth_ok_fills_its_cap() {
    let names: Vec<Vec<u8>> =
        (0..nsh_wire::MAX_CAPS).map(|i| format!("C{i:031}").into_bytes()).collect();
    let caps: Vec<&[u8]> = names.iter().map(|n| &n[..]).collect();
    let mut body = vec![0u8; MAX_AUTH_OK + 1];
    let n = AuthOk::encode(&[0x5E; 16], 600, &caps, &mut body).expect("the largest AUTH_OK");
    assert_eq!(n, MAX_AUTH_OK);
    assert!(AuthOk::parse(&body[..n]).is_ok());
    let inner = framed(Kind::AuthOk, &body[..n]);
    assert_eq!(inner.len(), Expect::AuthResult.cap());
    assert!(split(&inner, Expect::AuthResult).is_ok());
    one_more_is_refused(&inner, Expect::AuthResult);
}

#[test]
fn a_refusal_and_an_abort_fit_the_auth_result_cap() {
    for kind in [Kind::AuthRefused, Kind::Abort] {
        assert!(split(&framed(kind, &[1, 0]), Expect::AuthResult).is_ok(), "{kind:?}");
    }
    assert!(split(&framed(Kind::Abort, &[1, 0]), Expect::UserAuth).is_ok());
}

#[test]
fn an_escalation_in_user_auth_is_still_refused_by_the_parser() {
    let body = user_auth(&largest_grant().bytes());
    assert_eq!(UserAuth::parse(&body), Err(WireError::Value));
    assert!(body.len() + 1 > Expect::UserAuth.cap(), "the receiver refuses it before reading");
}

#[test]
fn the_largest_escalate_fits_the_session_cap() {
    let body = escalate(&largest_grant().bytes());
    assert!(nsh_wire::escalate(&body).is_ok());
    assert!(split(&framed(Kind::Escalate, &body), Expect::ServerSession).is_ok());
}
