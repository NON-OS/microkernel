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

//! A global NVS area: one SystemMemory region and 48 fields of mixed
//! widths over bytes laid down in a known pattern, with the sum expected.

use super::more::ADD;
use crate::enc::*;
use crate::rig::Rig;

pub const BASE: u64 = 0x7FF0_0000;
pub const LEN: u64 = 0x100;
const WIDTHS: [usize; 4] = [8, 16, 32, 8];

pub fn nvs() -> (Rig, u64) {
    let units: Vec<Vec<u8>> = (0..48).map(|i| unit(&format!("F{i:03}"), WIDTHS[i % 4])).collect();
    let mut sum = int(0);
    for i in 0..48 {
        sum = e(ADD, &[&sum, &name(&format!("F{i:03}")), &[0x00]]);
    }
    let write = cat(&[&store(&int(0x7DD), &name("F001")), &ret(&name("F001"))]);
    let defs = cat(&[
        &region("GNVS", 0, BASE, LEN),
        &field("GNVS", 0x00, &units),
        &method("SUMN", 0, &ret(&sum)),
        &method("WOSY", 0, &write),
    ]);
    let mut r = Rig::new(&defs).connected();
    let mut expected = 0u64;
    let mut at = 0u64;
    for i in 0..48 {
        let bytes = (WIDTHS[i % 4] / 8) as u64;
        let mut v = 0u64;
        for b in 0..bytes {
            let byte = ((at + b) * 7 + 3) as u8;
            r.probe.bytes.insert((0, BASE + at + b), byte);
            v |= u64::from(byte) << (8 * b);
        }
        expected = expected.wrapping_add(v);
        at += bytes;
    }
    (r, expected)
}
