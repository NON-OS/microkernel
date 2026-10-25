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

//! Sealed frames out of the handshake's order, unknown, oversize, empty,
//! changed or replayed. Each run is the good one up to `HOST_PROOF`, with
//! that one frame replaced. A frame that opens but is wrong is answered
//! `ABORT(PROTOCOL)`; one that does not open is answered with nothing.
//! None of them is ever followed by a credential.

use super::aborted_with;
use crate::kit::pair::Pair;
use crate::kit::server::Server;
use nsh_client::Refusal;
use nsh_wire::{Kind, Reason, WireError};

fn at_proof(frame: impl FnOnce(&mut Server) -> Vec<u8>) -> (Result<(), Refusal>, Pair) {
    let mut p = Pair::good();
    p.to_proof();
    let f = frame(&mut p.server);
    (p.feed(&f), p)
}

#[test]
fn auth_ok_in_place_of_host_proof_is_refused_and_no_credential_follows() {
    let (r, mut p) = at_proof(|s| s.seal(Kind::AuthOk, &Server::auth_ok_body()));
    assert_eq!(r, Err(Refusal::Wire(WireError::Order)));
    aborted_with(&mut p, Reason::Protocol);
    assert_eq!(p.client.attester().calls.get(), 0);
    assert_eq!(p.client.input(b"x"), Err(Refusal::NotNow));
}

#[test]
fn every_other_known_type_in_place_of_host_proof_is_out_of_order() {
    for kind in
        [0x01u8, 0x02, 0x20, 0x21, 0x22, 0x30, 0x31, 0x32, 0x33, 0x40, 0x41, 0x50, 0x51, 0x52]
    {
        let (r, mut p) = at_proof(|s| s.seal_inner(&[kind, 0, 0]));
        assert_eq!(r, Err(Refusal::Wire(WireError::Order)), "type {kind:#x}");
        aborted_with(&mut p, Reason::Protocol);
    }
}

#[test]
fn an_unknown_type_is_refused_as_unknown() {
    for kind in [0x00u8, 0x03, 0x11, 0x23, 0x60, 0x7E, 0x80, 0xFF] {
        let (r, mut p) = at_proof(|s| s.seal_inner(&[kind, 1]));
        assert_eq!(r, Err(Refusal::Wire(WireError::UnknownType)));
        aborted_with(&mut p, Reason::Protocol);
    }
}

#[test]
fn an_oversize_length_is_refused_before_its_body_and_an_empty_frame_too() {
    let over = (nsh_wire::AUTH_CAP as u32 + 16 + 1).to_le_bytes();
    let (r, mut p) = at_proof(|_| over.to_vec());
    assert_eq!(r, Err(Refusal::Wire(WireError::Oversize)));
    aborted_with(&mut p, Reason::Protocol);
    let (r, _) = at_proof(|_| 16u32.to_le_bytes().to_vec());
    assert_eq!(r, Err(Refusal::Wire(WireError::Short)));
}
