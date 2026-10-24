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

//! The checks run in the order roles.md gives: the key is proved before
//! anything else is told, and the second part runs check by check.

use crate::build::grant::GrantB;
use crate::build::keys::{key, K};
use crate::build::revoke::RevB;
use crate::build::run::Setup;
use crate::build::user_auth::UserAuthB;
use crate::build::{BASE_ID, HOUR, NOW};
use nsh_roles::Refusal;

/// Every second-part failure at once: another fleet, expired, a stale list
/// that names it, an unknown role and an unknown capability.
fn all_wrong() -> GrantB {
    GrantB {
        fleet: [0xF2; 16],
        not_after_ms: NOW,
        role: b"root".to_vec(),
        caps: vec!["Root"],
        ..GrantB::default()
    }
}

fn stale() -> Setup {
    let rev = RevB { issued_ms: NOW - 48 * HOUR, ids: vec![BASE_ID], ..RevB::default() };
    Setup { rev, ..Setup::default() }
}

#[test]
fn the_proof_of_the_key_comes_first() {
    let s = stale();
    let g = GrantB { issuer: key(K::Stranger).id(), signer: K::Stranger, ..all_wrong() };
    assert_eq!(s.login(&g).err(), Some(Refusal::Issuer));
    let g = GrantB { signer: K::AuthB, ..all_wrong() };
    assert_eq!(s.login(&g).err(), Some(Refusal::IssuerSig));
    let g = GrantB { subject: key(K::OtherUser).subject(), ..all_wrong() };
    assert_eq!(s.login(&g).err(), Some(Refusal::Subject));
    let ua = UserAuthB { signer: K::OtherUser, ..UserAuthB::new(all_wrong().bytes()) };
    assert_eq!(s.login_ua(&ua).err(), Some(Refusal::UserSig));
}

#[test]
fn the_second_part_runs_in_order() {
    let s = stale();
    let mut g = all_wrong();
    assert_eq!(s.login(&g).err(), Some(Refusal::Fleet));
    g.fleet = GrantB::default().fleet;
    assert_eq!(s.login(&g).err(), Some(Refusal::Expired));
    g.not_after_ms = NOW + HOUR;
    assert_eq!(s.login(&g).err(), Some(Refusal::RevocationStale));
    let fresh = Setup { rev: RevB { ids: vec![BASE_ID], ..RevB::default() }, ..s };
    assert_eq!(fresh.login(&g).err(), Some(Refusal::Revoked));
    let clean = Setup::default();
    assert_eq!(clean.login(&g).err(), Some(Refusal::RoleUnknown));
    g.role = b"operator".to_vec();
    assert_eq!(clean.login(&g).err(), Some(Refusal::CapUnknown));
    g.caps = vec!["CoreExec"];
    assert!(clean.login(&g).is_ok());
}
