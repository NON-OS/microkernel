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

//! Panels as driver.bt_usb0 would have left them, and its payloads.

use crate::bluetooth::panel::{Panel, Reach};
use nonos_bt_proto::{Asked, Link, NO_PLACE};

pub const KB: [u8; 6] = [0x11, 0x22, 0x33, 0x44, 0x55, 0xc6];
pub const MOUSE: [u8; 6] = [0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb];

/// A panel the service answered, with LINK's `state` and `flags`.
pub fn answered(state: u8, flags: u8) -> Panel {
    let mut p = Panel::new();
    p.reach = Reach::Answered;
    assert!(p.took_link(&Link { state, flags, place: NO_PLACE }.write()));
    p
}

pub fn asked(p: &mut Panel, a: Asked) {
    assert!(p.took_asked(&a.write()));
}

/// A BONDS entry.
pub fn bond(kind: u8, addr: [u8; 6], authenticated: bool, linked: bool) -> Vec<u8> {
    [&[kind][..], &addr, &[u8::from(authenticated), u8::from(linked)]].concat()
}

/// A counted list of entries.
pub fn list(entries: &[Vec<u8>]) -> Vec<u8> {
    [vec![entries.len() as u8], entries.concat()].concat()
}

/// The keyboard, authenticated, and the mouse, linked.
pub fn two_bonds(p: &mut Panel) {
    assert!(p.took_bonds(&list(&[bond(1, KB, true, false), bond(0, MOUSE, false, true)])));
}
