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

//! An escalation must be an escalation grant, from a role authority whose
//! signatures both verify; a failure is told as `GRANT_SIG`, since this
//! session's user is already known.

use super::escalate::esc;
use crate::build::grant::GrantB;
use crate::build::keys::{key, K};
use crate::build::Spoil;
use nsh_roles::Refusal;
use nsh_wire::Reason;

#[test]
fn a_base_grant_is_not_an_escalation() {
    assert_eq!(esc(GrantB::default()), Err(Refusal::Kind));
}

#[test]
fn the_issuer_must_be_an_authority_whose_signatures_both_verify() {
    let k = K::Stranger;
    let g = GrantB { issuer: key(k).id(), signer: k, ..GrantB::escalation() };
    assert_eq!(esc(g), Err(Refusal::GrantSig));
    assert_eq!(esc(GrantB { signer: K::AuthB, ..GrantB::escalation() }), Err(Refusal::GrantSig));
    for spoil in [Spoil::Ed, Spoil::Ml] {
        assert_eq!(esc(GrantB { spoil, ..GrantB::escalation() }), Err(Refusal::GrantSig));
    }
    assert_eq!(Refusal::GrantSig.reason(), Reason::GrantSig);
}
