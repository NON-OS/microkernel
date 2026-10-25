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

//! Escalation answers and sealed ABORTs once the session runs.

use super::aborted_with;
use super::check::{gets, logged_in};
use crate::kit::grant;
use nsh_client::{Event, Refusal};
use nsh_wire::{Kind, Reason, WireError};

#[test]
fn an_escalation_answer_must_answer_an_escalation() {
    let granted = [&0u16.to_le_bytes()[..], &77u64.to_le_bytes()].concat();
    let mut p = logged_in();
    assert_eq!(gets(&mut p, Kind::EscalateResult, &granted), Err(Refusal::Wire(WireError::Order)));
    aborted_with(&mut p, Reason::Protocol);
    let mut p = logged_in();
    p.client.escalate(&grant_body()).unwrap();
    assert_eq!(gets(&mut p, Kind::EscalateResult, &granted), Ok(()));
    let ev = Event::Escalated { reason: Reason::Ok, expires_ms: 77 };
    assert_eq!(p.client.events(), vec![ev]);
    assert_eq!(gets(&mut p, Kind::EscalateResult, &granted), Err(Refusal::Wire(WireError::Order)));
    let mut p = logged_in();
    p.client.escalate(&grant_body()).unwrap();
    let refused_ok = [&33u16.to_le_bytes()[..], &77u64.to_le_bytes()].concat();
    assert_eq!(
        gets(&mut p, Kind::EscalateResult, &refused_ok),
        Err(Refusal::Wire(WireError::Value))
    );
}

fn grant_body() -> Vec<u8> {
    grant::escalation()
}

#[test]
fn a_sealed_abort_ends_the_session_with_its_reason() {
    for code in 1u16..=5 {
        let mut p = logged_in();
        let r = gets(&mut p, Kind::Abort, &code.to_le_bytes());
        assert_eq!(r, Err(Refusal::Aborted(Reason::from_code(code).unwrap())));
        super::silent(&mut p);
    }
    let mut p = logged_in();
    assert_eq!(
        gets(&mut p, Kind::Abort, &0u16.to_le_bytes()),
        Err(Refusal::Wire(WireError::Value))
    );
}
