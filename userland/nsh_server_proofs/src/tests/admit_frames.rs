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

//! The frames around the login that the real client never sends, and the
//! host's answer to an admitted login: `AUTH_OK` names what the session
//! capsule was given, never more than the grant, and a host that cannot
//! start the session says `BUSY`.

use super::admit::*;
use crate::kit::raw_run::RawRun;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::K;
use nsh_roles::Refusal as R;
use nsh_server::Why;
use nsh_wire::{Kind, Reason, WireError};

#[test]
fn a_broken_user_auth_is_answered_protocol() {
    let mut r = RawRun::proved();
    let mut ua = r.raw.user_auth(&GrantB::default(), K::User);
    ua.pop();
    let f = r.raw.seal(Kind::UserAuth, &ua);
    assert_eq!(told(&r.frames(&f)), [PROTOCOL]);
}

/// An escalation carries at least one approval, so a `USER_AUTH` holding
/// one is longer than the stage's cap: it is refused at its length field,
/// before any of it is kept, and the parser's own refusal stays behind it.
#[test]
fn an_escalation_in_user_auth_is_refused_before_it_is_read() {
    let mut r = RawRun::proved();
    let ua = r.raw.user_auth(&GrantB::escalation(), K::User);
    assert!(1 + ua.len() > nsh_wire::USER_AUTH_CAP);
    assert_eq!(nsh_wire::UserAuth::parse(&ua), Err(WireError::Value));
    let f = r.raw.seal(Kind::UserAuth, &ua);
    assert_eq!(told(&r.frames(&f)), [PROTOCOL]);
    assert_eq!(r.refused(), [Why::Wire(WireError::Oversize)]);
}

#[test]
fn a_key_that_is_not_the_grant_subject_is_user_sig() {
    let mut r = RawRun::proved();
    let ua = r.raw.user_auth(&GrantB::default(), K::OtherUser);
    let f = r.raw.seal(Kind::UserAuth, &ua);
    assert_eq!(
        told(&r.frames(&f)),
        [(Kind::AuthRefused.byte(), Reason::UserSig.code().to_le_bytes())]
    );
    assert_eq!(r.refused(), [Why::Login(R::Subject)]);
}

#[test]
fn the_client_abort_is_not_answered() {
    let mut r = RawRun::proved();
    let f = r.raw.seal(Kind::Abort, &Reason::Policy.code().to_le_bytes());
    assert!(r.send(&f).is_empty());
    assert_eq!(r.refused(), [Why::Aborted(Reason::Policy)]);
    let mut r = RawRun::proved();
    let f = r.raw.seal(Kind::Abort, &Reason::GrantSig.code().to_le_bytes());
    assert_eq!(told(&r.frames(&f)), [PROTOCOL], "a code an ABORT may not carry");
}
