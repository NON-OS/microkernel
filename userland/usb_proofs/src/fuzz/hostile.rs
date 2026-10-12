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

//! The configuration descriptors the fuzz plays a hostile device with.

use crate::config_blob::{config, HID, INTERRUPT};
use crate::rng::Rng;

/// A configuration a hostile device might send: plausible records mixed
/// with cut ones, records of any length and type, and a wTotalLength that
/// may or may not match.
pub fn hostile_config(rng: &mut Rng) -> Vec<u8> {
    let mut blob = config();
    for _ in 0..rng.below(12) {
        let r = rng.byte();
        blob = match rng.below(8) {
            0..=2 => {
                let class = rng.pick(&[HID, HID, HID, 0x08, r]);
                let subclass = rng.pick(&[0, 1, 1, 2, r]);
                let protocol = rng.pick(&[0, 1, 2, 3, r]);
                blob.iface(rng.byte(), class, subclass, protocol)
            }
            3..=5 => {
                let address = rng.pick(&[0x81, 0x82, 0x8f, 0x80, 0x01, r]);
                let attributes = rng.pick(&[INTERRUPT, INTERRUPT, 0x02, r]);
                let w = rng.word() as u16;
                let size = rng.pick(&[0, 2, 3, 4, 5, 7, 8, 9, 64, 1024, 1025, 0x7ff, 0x1808, w]);
                blob.ep(address, attributes, size, rng.byte())
            }
            6 => blob.hid(),
            _ => {
                let len = rng.pick(&[0, 1, 2, 3, 6, 7, 8, 9, r]);
                let mut rec = vec![len, rng.pick(&[0x04, 0x05, 0x21, r])];
                let body = rng.below(12);
                rec.extend(rng.bytes(body));
                blob.raw(&rec)
            }
        };
    }
    let size = blob.size() as u16;
    let total = match rng.below(8) {
        0 => rng.word() as u16,
        1 => size.saturating_sub(rng.below(16) as u16),
        2 => size.saturating_add(rng.below(16) as u16),
        _ => size,
    };
    let mut d = blob.build_with_total(total);
    if rng.one_in(16) {
        d.truncate(rng.below(d.len() + 1));
    }
    d
}
