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

//! A controller that stops answering mid-link: the first command after
//! that, here the check of a quiet link, waits its poll budget and ends
//! the controller's run with `Timeout`; the key held down goes up. A flood
//! of events the link has no use for, with counts of buffers for a link
//! that holds none among them, is taken one a turn beside the peer's data.

use super::{A_DOWN, KEYS};
use crate::air::{Act, Lies};
use crate::fixture::{air, keyboard, link};
use crate::hci_sim::events::{adv_report, completed};
use crate::link::End;
use nonos_bt_hci::host::READ_RSSI;
use nonos_bt_hci::HciError;
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn a_controller_that_stops_answering_ends_its_run_in_bounded_time() {
    let (mut host, ours, mut env) = keyboard();
    let mute = Act::Lie(|l: &mut Lies| l.mute = true);
    air(&mut host).script.extend([Act::Notify(KEYS, A_DOWN.to_vec()), mute]);
    let r = link(&mut host, &ours, &mut env).map(|_| ());
    assert_eq!(r, Err(HciError::Timeout(READ_RSSI)));
    assert_eq!(env.input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
    assert!(env.now < 30_000, "ended at {} ms", env.now);
}

/// New timing a peripheral asks for, which the controller never answers:
/// that is no refusal, and it ends the controller's run.
#[test]
fn a_connection_update_left_unanswered_ends_the_run() {
    let (mut host, ours, mut env) = keyboard();
    let ask = vec![0x12, 7, 8, 0, 0x18, 0, 0x28, 0, 0, 0, 0xf4, 0x01];
    let script = [Act::Lie(|l: &mut Lies| l.mute = true), Act::Signal(ask)];
    air(&mut host).script.extend(script);
    let r = link(&mut host, &ours, &mut env).map(|_| ());
    assert_eq!(r, Err(HciError::Timeout(op::LE_CONNECTION_UPDATE)));
}

#[test]
fn a_flood_of_events_is_taken_beside_the_peers_data() {
    let (mut host, ours, mut env) = keyboard();
    let flood: Vec<Vec<u8>> =
        (0..3000).flat_map(|i| [adv_report(i as u8, &[]), completed(0x0041, 9)]).collect();
    let script = [
        Act::Events(flood),
        Act::Notify(KEYS, A_DOWN.to_vec()),
        Act::Notify(KEYS, vec![0; 8]),
        Act::Drop(0x13),
    ];
    air(&mut host).script.extend(script);
    let total = host.acl_free();
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(env.input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
    assert_eq!(host.acl_free(), total, "no count for another link gave more");
}
