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
//! The interleaving sweep's packets: key presses as notifications, cut
//! into fragments, with another link's slipped between them.

use crate::air::HANDLE;
use nonos_bt_wire::hci::{acl, Boundary};
use nonos_bt_wire::l2cap::{basic, cid::ATT};

/// The boot keyboard input report's value handle in `keyboard_mouse`.
pub const KEYS: u16 = 11;
pub const SEEDS: [u64; 4] = [0x9e37_79b9_7f4a_7c15, 0x2545_f491_4f6c_dd1d, 7, 0xdead_beef];

pub struct Rng(pub u64);

impl Rng {
    pub fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}

pub fn keys(key: u8) -> Vec<u8> {
    basic(ATT, &[&[0x1b][..], &KEYS.to_le_bytes(), &[0, 0, key, 0, 0, 0, 0, 0]].concat()).unwrap()
}

/// `pdu` cut into fragments of up to 27 octets on `handle`.
pub fn cut(rng: &mut Rng, handle: u16, pdu: &[u8]) -> Vec<Vec<u8>> {
    let (mut out, mut at) = (Vec::new(), 0);
    while at < pdu.len() {
        let n = (1 + rng.below(27)).min(pdu.len() - at);
        let b = if at == 0 { Boundary::FirstFlushable } else { Boundary::Continuing };
        out.push(acl(handle, b, &pdu[at..at + n]).unwrap());
        at += n;
    }
    out
}

/// `ours` with another link's fragments, a whole key press among them,
/// slipped in at random.
pub fn between(rng: &mut Rng, ours: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for p in ours {
        let other = loop {
            match rng.below(0x0f00) as u16 {
                h if h != HANDLE => break h,
                _ => continue,
            }
        };
        if rng.below(2) == 0 {
            out.extend(cut(rng, other, &keys(0x05)));
        }
        out.push(p);
    }
    out
}
