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

//! What the client receives once logged in. Output only while the shell
//! runs; an escalation answer only for an escalation asked; a frame of a
//! kind the server never sends, or a body out of range, is refused with
//! `ABORT(PROTOCOL)`; a sealed `ABORT` ends the session with its reason.

use super::aborted_with;
use super::check::{gets, logged_in};
use nsh_client::{Event, Refusal};
use nsh_wire::{Kind, Reason, WireError};

#[test]
fn frames_the_server_never_sends_are_refused() {
    for kind in [Kind::Input, Kind::Window, Kind::Signal, Kind::Escalate, Kind::HostProof] {
        let mut p = logged_in();
        assert_eq!(gets(&mut p, kind, &[1]), Err(Refusal::Wire(WireError::Order)));
        aborted_with(&mut p, Reason::Protocol);
    }
    let mut p = logged_in();
    assert_eq!(gets(&mut p, Kind::AuthOk, &[1]), Err(Refusal::Wire(WireError::Order)));
}

#[test]
fn output_is_held_to_its_size() {
    let mut p = logged_in();
    assert_eq!(gets(&mut p, Kind::Output, &[]), Err(Refusal::Wire(WireError::Value)));
    aborted_with(&mut p, Reason::Protocol);
    let mut p = logged_in();
    let f = p.server.seal_inner(&[&[Kind::Output.byte()][..], &[0; 16_385]].concat());
    assert_eq!(p.feed(&f), Err(Refusal::Wire(WireError::Oversize)));
    let mut p = logged_in();
    assert_eq!(gets(&mut p, Kind::Output, &[7; 16_384]), Ok(()));
    assert_eq!(p.client.events(), vec![Event::Output(vec![7; 16_384])]);
}
