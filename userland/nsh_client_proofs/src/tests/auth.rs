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

//! `USER_AUTH` and its answer. The client's signature covers the
//! transcript with the accepted `HOST_PROOF` in it, so a server holding
//! another transcript refuses it. `AUTH_REFUSED` ends the session with
//! the server's reason; a reason or body it may not carry is refused. A
//! grant that is not a base grant for the user's key is never sent.

use super::{aborted_with, silent};
use crate::kit::pair::Pair;
use crate::kit::server::Server;
use nsh_client::Refusal;
use nsh_wire::{Kind, Reason, WireError};

#[test]
fn the_user_signature_is_bound_to_this_transcript() {
    let mut p = Pair::good();
    p.to_proof();
    let hp = p.proof(&crate::kit::proof::ProofSpec::good());
    p.feed(&hp).unwrap();
    let ua = p.sent().remove(0).1;
    let mut other = Server::default();
    other.th.push(Kind::ClientHello, b"another session");
    assert!(!other.user_auth(&ua), "another transcript refuses it");
    assert!(p.server.user_auth(&ua));
}

pub(super) fn answer(body_kind: Kind, body: &[u8]) -> (Result<(), Refusal>, Pair) {
    let mut p = Pair::good();
    p.to_auth();
    let f = p.server.seal(body_kind, body);
    (p.feed(&f), p)
}

#[test]
fn auth_refused_ends_the_session_with_its_reason() {
    for code in 16u16..=24 {
        let (r, mut p) = answer(Kind::AuthRefused, &code.to_le_bytes());
        assert_eq!(r, Err(Refusal::AuthRefused(Reason::from_code(code).unwrap())));
        silent(&mut p);
    }
    for code in [0u16, 1, 3, 5, 25, 32, 999] {
        let (r, mut p) = answer(Kind::AuthRefused, &code.to_le_bytes());
        assert_eq!(r, Err(Refusal::Wire(WireError::Value)), "code {code}");
        aborted_with(&mut p, Reason::Protocol);
    }
}
