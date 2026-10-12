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

//! The LE signaling channel (Vol 3, Part A, 4): new timing a peripheral
//! asks for is answered, and applied with LE Connection Update when it is
//! within range; anything else this host does not do is rejected.

use crate::air::Act;
use crate::fixture::{air, keyboard, link};
use nonos_bt_wire::hci::opcode as op;

/// A Connection Parameter Update Request (4.20), identifier 7: interval
/// 30 to 50 ms, `latency`, 5 s supervision timeout.
fn update_request(latency: u16) -> Vec<u8> {
    let mut f = vec![0x12, 7, 8, 0, 0x18, 0, 0x28, 0];
    f.extend_from_slice(&latency.to_le_bytes());
    f.extend_from_slice(&0x01f4u16.to_le_bytes());
    f
}

fn updates(commands: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let o = op::LE_CONNECTION_UPDATE.0.to_le_bytes();
    commands.iter().filter(|c| c[..2] == o).cloned().collect()
}

#[test]
fn timing_in_range_is_accepted_then_applied() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.extend([Act::Signal(update_request(4)), Act::Drop(0x13)]);
    link(&mut host, &ours, &mut env).unwrap();
    let a = air(&mut host);
    assert_eq!(a.signals, [vec![0x13, 7, 2, 0, 0, 0]], "accepted");
    let want = [0x13, 0x20, 14, 0x40, 0, 0x18, 0, 0x28, 0, 4, 0, 0xf4, 0x01, 0, 0, 0, 0];
    assert_eq!(updates(&a.commands), [want.to_vec()]);
}

#[test]
fn timing_out_of_range_is_rejected_and_not_applied() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.extend([Act::Signal(update_request(0x01f4)), Act::Drop(0x13)]);
    link(&mut host, &ours, &mut env).unwrap();
    let a = air(&mut host);
    assert_eq!(a.signals, [vec![0x13, 7, 2, 0, 1, 0]], "rejected");
    assert!(updates(&a.commands).is_empty());
}

#[test]
fn a_request_this_host_does_not_serve_is_rejected() {
    // An LE Credit Based Connection Request (4.22): this host opens no
    // dynamic channel, so the answer is Command Reject, not understood.
    let open = vec![0x14, 9, 10, 0, 0x80, 0, 0x40, 0, 0x17, 0, 0x17, 0, 1, 0];
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.extend([Act::Signal(open), Act::Drop(0x13)]);
    link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(air(&mut host).signals, [vec![0x01, 9, 2, 0, 0, 0]]);
}
