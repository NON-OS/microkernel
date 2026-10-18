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

//! LE Connection Complete while the link is up, which this host never asked
//! for. Naming the link's own handle, the controller has given it to
//! another link and the link ends, its key let go; naming another handle,
//! that link is ended with Disconnect and this one goes on.

use super::{opened, A_DOWN, KEYS};
use crate::air::{Act, HANDLE};
use crate::fixture::{air, keyboard, link};
use crate::link::{hang_up, End};
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn an_open_links_handle_given_to_another_ends_the_link() {
    let (mut host, us, mut env) = keyboard();
    let script = [Act::Notify(KEYS, A_DOWN.to_vec()), Act::Event(opened(HANDLE))];
    air(&mut host).script.extend(script);
    let (end, _) = link(&mut host, &us, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("the controller gave this link's handle to another"));
    assert_eq!(env.input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
    hang_up(&mut host, HANDLE).unwrap();
    let o = op::DISCONNECT.0.to_le_bytes();
    let cmds = &air(&mut host).commands;
    assert_eq!(cmds.iter().filter(|c| c[..2] == o && c[3..5] == HANDLE.to_le_bytes()).count(), 1);
}

#[test]
fn a_link_no_one_asked_for_is_ended_and_this_one_goes_on() {
    let (mut host, us, mut env) = keyboard();
    let script = [
        Act::Event(opened(0x0041)),
        Act::Notify(KEYS, A_DOWN.to_vec()),
        Act::Notify(KEYS, vec![0; 8]),
        Act::Drop(0x13),
    ];
    air(&mut host).script.extend(script);
    let (end, _) = link(&mut host, &us, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(env.input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
    let o = op::DISCONNECT.0.to_le_bytes();
    let cmds = &air(&mut host).commands;
    assert!(cmds.iter().any(|c| c[..2] == o && c[3..5] == [0x41, 0x00]), "the stray ended");
}
