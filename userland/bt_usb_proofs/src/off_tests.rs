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

//! The radio turned off ends a live link: nothing more is sent on it, the
//! open handle is disconnected (Vol 4, Part E, 7.1.6), and that disconnect
//! is given up at its own time bound when the controller never completes it.

use crate::air::Act;
use crate::fixture::{air, keyboard, link};
use crate::link::{hang_up, End, OFF};
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

/// A Connection Parameter Update Request (Vol 3, Part A, 4.20) a served
/// link would answer on the signaling channel.
const UPDATE: [u8; 12] = [0x12, 7, 8, 0, 0x18, 0, 0x28, 0, 4, 0, 0xf4, 0x01];

#[test]
fn off_ends_a_live_link_and_answers_nothing_on_it() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.extend([Act::Signal(UPDATE.to_vec()), Act::Drop(0x13)]);
    env.user.off = true;
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp(OFF));
    assert!(air(&mut host).signals.is_empty(), "no answer on the link after off");
}

#[test]
fn off_disconnects_the_open_handle() {
    let (mut host, ours, mut env) = keyboard();
    env.user.off = true;
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp(OFF));
    assert_eq!(hang_up(&mut host, s.handle), Ok(()));
    let last = air(&mut host).commands.last().unwrap().clone();
    assert_eq!(last[..2], op::DISCONNECT.0.to_le_bytes());
    assert_eq!(last[3..], [0x40, 0x00, 0x13], "handle, Remote User Terminated");
}

#[test]
fn a_disconnect_never_completed_is_given_up_at_the_bound() {
    let (mut host, ours, mut env) = keyboard();
    env.user.off = true;
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp(OFF));
    air(&mut host).lies.linger = true;
    let out = hang_up(&mut host, s.handle);
    assert_eq!(out, Err(HciError::Timeout(op::DISCONNECT)), "bounded, not forever");
}
