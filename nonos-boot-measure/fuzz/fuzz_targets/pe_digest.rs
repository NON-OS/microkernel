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

//! The PE Authenticode digest on any bytes: no panic, and a digest that exists
//! does not move when the checksum field is rewritten, unless a section's raw
//! data covers that field, as EDK2 then hashes it too.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_boot_measure::authenticode::digest;

fn u(f: &[u8], at: usize, n: usize) -> usize {
    f.get(at..at + n).map_or(0, |b| b.iter().rev().fold(0, |a, &x| a << 8 | usize::from(x)))
}

fuzz_target!(|f: &[u8]| {
    let Ok(d) = digest(f) else { return };
    let opt = u(f, 0x3C, 4) + 24;
    let cs = opt + 64;
    let table = opt + u(f, opt - 4, 2);
    let covered = (0..u(f, opt - 18, 2)).any(|i| {
        let (size, at) = (u(f, table + 40 * i + 16, 4), u(f, table + 40 * i + 20, 4));
        size > 0 && at < cs + 4 && cs < at + size
    });
    let mut g = f.to_vec();
    if let (Some(field), false) = (g.get_mut(cs..cs + 4), covered) {
        field.copy_from_slice(&[0xA5; 4]);
        assert_eq!(digest(&g), Ok(d));
    }
});
