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

//! A link whose encryption the controller says is gone ends, its key let
//! go: Encryption Change off (Vol 4, Part E, 7.7.8), or Encryption Key
//! Refresh Complete (7.7.39) with a failure.

use super::{A_DOWN, KEYS};
use crate::air::{Act, HANDLE};
use crate::fixture::{air, keyboard, link};
use crate::hci_sim::link_events::encryption_change;
use crate::link::End;
use nonos_bt_hogp::report::Input;

const LOST: End = End::GaveUp("the controller says the link is no longer encrypted");

fn ends(event: Vec<u8>) {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.extend([Act::Notify(KEYS, A_DOWN.to_vec()), Act::Event(event)]);
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, LOST);
    assert_eq!(env.input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
}

#[test]
fn encryption_turned_off_or_a_failed_refresh_ends_the_link() {
    ends(encryption_change(0, HANDLE, 0));
    ends(encryption_change(0x3d, HANDLE, 1));
    let [h0, h1] = HANDLE.to_le_bytes();
    ends(vec![0x30, 3, 0x3d, h0, h1]);
}

/// The peer's request arrives in the turn that learns encryption is gone:
/// its answer is not sent.
#[test]
fn nothing_is_answered_once_encryption_is_known_gone() {
    let (mut host, ours, mut env) = keyboard();
    let ask = vec![0x12, 7, 8, 0, 0x18, 0, 0x28, 0, 0, 0, 0xf4, 0x01];
    air(&mut host).script.push_back(Act::Together(encryption_change(0, HANDLE, 0), ask));
    assert_eq!(link(&mut host, &ours, &mut env).unwrap().0, LOST);
    assert!(air(&mut host).signals.is_empty(), "{:02x?}", air(&mut host).signals);
}

#[test]
fn another_links_encryption_or_a_refresh_that_worked_ends_nothing() {
    let (mut host, ours, mut env) = keyboard();
    let [h0, h1] = HANDLE.to_le_bytes();
    let script = [
        Act::Event(encryption_change(0, 0x0041, 0)),
        Act::Event(vec![0x30, 3, 0, h0, h1]),
        Act::Drop(0x13),
    ];
    air(&mut host).script.extend(script);
    assert_eq!(link(&mut host, &ours, &mut env).unwrap().0, End::Lost(0x13));
}
