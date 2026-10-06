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

//! A device pulled out of a root port takes its endpoints with it, and each
//! slot it held is handed back exactly once: a slot given back twice would
//! free one the controller driver has since given to the next device.

use crate::descriptors::HidKind;
use crate::enumerate::drop_port::drop_port;
use crate::enumerate::types::HidEndpoint;

fn ep(root_port: u8, slot: u8, dci: u8, kind: HidKind) -> HidEndpoint {
    HidEndpoint { port: 7, root_port, slot, dci, kind, max_packet: 8 }
}

#[test]
fn a_receiver_with_a_keyboard_and_a_mouse_gives_back_its_one_slot_once() {
    let mut eps = vec![
        ep(3, 1, 3, HidKind::Keyboard),
        ep(3, 1, 5, HidKind::Mouse),
        ep(4, 2, 3, HidKind::Mouse),
    ];
    assert_eq!(drop_port(&mut eps, 3), vec![1]);
    assert_eq!(eps.len(), 1);
    assert_eq!((eps[0].root_port, eps[0].slot), (4, 2));
}

#[test]
fn an_empty_port_with_nothing_bound_gives_back_nothing() {
    let mut eps = vec![ep(4, 2, 3, HidKind::Keyboard)];
    assert!(drop_port(&mut eps, 9).is_empty());
    assert_eq!(eps.len(), 1);
}

#[test]
fn devices_behind_one_root_port_give_back_every_slot_each_once() {
    let mut eps = vec![
        ep(2, 5, 3, HidKind::Keyboard),
        ep(2, 6, 3, HidKind::Mouse),
        ep(2, 5, 5, HidKind::Tablet),
        ep(1, 4, 3, HidKind::Mouse),
    ];
    assert_eq!(drop_port(&mut eps, 2), vec![5, 6]);
    assert!(eps.iter().all(|e| e.root_port == 1));
    assert!(drop_port(&mut eps, 2).is_empty());
}
