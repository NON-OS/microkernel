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
//! The HID interfaces that could be a security key: alternate 0 of class
//! 03h, with an interrupt IN and an interrupt OUT endpoint of 64-byte
//! packets, CTAPHID's report size. Its report descriptor (bind.rs) decides.

use alloc::vec::Vec;

use nonos_usbnet::desc::{walk, ENDPOINT, INTERFACE};
use nonos_usbnet::Notify;

use super::frame::PACKET;

const CLASS_HID: u8 = 0x03;
const INTERRUPT: u8 = 0x03;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Key {
    pub interface: u8,
    pub input: Notify,
    pub output: Notify,
}

pub fn find_keys(raw: &[u8]) -> Vec<Key> {
    let (mut keys, mut at): (Vec<Key>, Option<Key>) = (Vec::new(), None);
    for d in walk(raw) {
        if d[1] == INTERFACE && d.len() >= 9 {
            keys.extend(at.take().filter(|k| k.input.address != 0 && k.output.address != 0));
            let hid = d[3] == 0 && d[5] == CLASS_HID;
            let none = Notify::default();
            at = hid.then_some(Key { interface: d[2], input: none, output: none });
        } else if let (Some(k), true) = (at.as_mut(), d[1] == ENDPOINT && d.len() >= 7) {
            let max_packet = u16::from_le_bytes([d[4], d[5]]);
            if d[3] & 0x3 != INTERRUPT || max_packet as usize != PACKET {
                continue;
            }
            let ep = Notify { address: d[2], max_packet, interval: d[6] };
            if d[2] & 0x80 != 0 {
                k.input = ep;
            } else {
                k.output = ep;
            }
        }
    }
    keys.extend(at.filter(|k| k.input.address != 0 && k.output.address != 0));
    keys
}
