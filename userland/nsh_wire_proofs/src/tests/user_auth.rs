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

//! `USER_AUTH` and `ESCALATE`: each carries one grant, judged as a grant,
//! and an `ESCALATE` grant must be an escalation.

use crate::build::grant::GrantB;
use crate::build::proof::{default_user_auth, escalate, user_auth};
use nsh_wire::{GrantKind, UserAuth, WireError};

#[test]
fn a_user_auth_places_its_parts() {
    let bytes = default_user_auth();
    let u = UserAuth::parse(&bytes).expect("valid");
    assert_eq!((u.user.ed[0], u.user.ml[1951]), (0x55, 0x55));
    assert_eq!((u.grant.kind, u.grant.user), (GrantKind::Base, &b"ek"[..]));
    assert_eq!((u.sig.ed[63], u.sig.ml[0]), (0x5A, 0x5A));
    assert_eq!(u.signed, &bytes[..bytes.len() - 3373]);
}

#[test]
fn a_bad_grant_inside_is_refused_as_the_grant_would_be() {
    let bad = GrantB { not_before: 5_000, ..GrantB::default() }.bytes();
    assert_eq!(UserAuth::parse(&user_auth(&bad)), Err(WireError::Value));
    assert_eq!(UserAuth::parse(&user_auth(&[])), Err(WireError::Value));
}

#[test]
fn a_user_auth_carries_a_base_grant_never_an_escalation() {
    let up = GrantB::escalation().bytes();
    assert_eq!(UserAuth::parse(&user_auth(&up)), Err(WireError::Value));
}

#[test]
fn a_cut_or_padded_user_auth_is_refused() {
    let bytes = default_user_auth();
    assert_eq!(UserAuth::parse(&bytes[..bytes.len() - 1]), Err(WireError::Short));
    assert_eq!(UserAuth::parse(&[bytes, vec![0]].concat()), Err(WireError::Trailing));
}

#[test]
fn escalate_takes_an_escalation_grant_only() {
    let up = GrantB::escalation().bytes();
    assert_eq!(nsh_wire::escalate(&escalate(&up)).map(|g| g.kind), Ok(GrantKind::Escalation));
    let base = GrantB::default().bytes();
    assert_eq!(nsh_wire::escalate(&escalate(&base)), Err(WireError::Value));
    let padded = [escalate(&up), vec![0]].concat();
    assert_eq!(nsh_wire::escalate(&padded), Err(WireError::Trailing));
}
