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

//! `USER_AUTH` and the role table: each refusal is answered `AUTH_REFUSED`
//! with the one code it is told as, and logged whole; nothing about the
//! grant is judged before the proof of the key.

use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::{key, K};
use crate::kit::roles_build::{Spoil, HOUR, NOW};
use crate::kit::run::Run;
use nsh_client::Refusal;
use nsh_roles::Refusal as R;
use nsh_server::Why;
use nsh_wire::Reason;

pub(super) fn refused(grant: GrantB, table: Table) -> (Reason, Vec<Why>) {
    let mut run = Run::new(&grant, table);
    run.pump();
    let Some(Refusal::AuthRefused(reason)) = run.c_err else { panic!("{:?}", run.c_err) };
    assert!(run.server.is_over() && run.client.is_over());
    assert!(run.c_events.iter().all(|e| !matches!(e, nsh_client::Event::LoggedIn(_))));
    (reason, run.refused())
}

pub(super) fn grant_refused(grant: GrantB, code: Reason, why: R) {
    assert_eq!(refused(grant, Table::shipped()), (code, vec![Why::Login(why)]));
}

#[test]
fn an_issuer_the_table_does_not_hold_is_user_sig() {
    let g = GrantB { issuer: key(K::Stranger).id(), signer: K::Stranger, ..GrantB::default() };
    grant_refused(g, Reason::UserSig, R::Issuer);
}

#[test]
fn either_half_of_the_issuer_signature_is_checked() {
    for spoil in [Spoil::Ed, Spoil::Ml] {
        grant_refused(GrantB { spoil, ..GrantB::default() }, Reason::UserSig, R::IssuerSig);
    }
}

#[test]
fn the_grant_window_is_held_to_the_host_clock() {
    let late = GrantB { not_before_ms: NOW + 1, not_after_ms: NOW + HOUR, ..GrantB::default() };
    grant_refused(late, Reason::GrantTime, R::NotYet);
    let gone = GrantB { not_before_ms: NOW - HOUR, not_after_ms: NOW, ..GrantB::default() };
    grant_refused(gone, Reason::GrantTime, R::Expired);
}

#[test]
fn another_fleet_is_out_of_scope() {
    grant_refused(GrantB { fleet: [0xF2; 16], ..GrantB::default() }, Reason::GrantScope, R::Fleet);
}
