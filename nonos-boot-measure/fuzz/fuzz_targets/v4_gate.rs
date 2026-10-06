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

//! The bootloader slot behind a v4 trailer, path and STARK, against the real
//! enrolled fixture: the input is a list of edits applied to the honest trailer.
//! Only the honest trailer itself may be admitted.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_boot_measure::authenticode::digest;
use nonos_boot_measure::gate::membership;

const TRAILER: &[u8] = include_bytes!("../../src/tests/fixture/trailer.bin");
const LOADER: &[u8] = include_bytes!("../../src/tests/fixture/loader.efi");
const ROOT: &[u8; 32] = include_bytes!("../../src/tests/fixture/root.bin");

/* Edits of five bytes each: a little-endian offset, then the byte to xor in;
 * an offset past the end truncates there instead. */
fn edited(data: &[u8]) -> Vec<u8> {
    let mut t = TRAILER.to_vec();
    for e in data.chunks_exact(5) {
        let at = u32::from_le_bytes([e[0], e[1], e[2], e[3]]) as usize;
        match t.get_mut(at % (TRAILER.len() + 64)) {
            Some(b) => *b ^= e[4].max(1),
            None => t.truncate(at % TRAILER.len()),
        }
    }
    t
}

fuzz_target!(|data: &[u8]| {
    let Ok(m) = digest(LOADER) else { return };
    let t = edited(data);
    if membership(ROOT, &m, &t).is_ok() {
        assert_eq!(t, TRAILER, "an edited trailer was admitted");
    }
});
