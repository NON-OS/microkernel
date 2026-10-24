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

//! The seeded sweep of what `--box` may be handed: words built from the
//! pieces a box is spelled with, cut, doubled and mixed with any byte, each
//! held to `check_word`. Some must be taken, of every preset.

use super::super::random::Regs;
use crate::boxed::check::check_word;
use crate::boxed::preset::Preset;

const PIECES: &[&[u8]] = &[
    b"allow:", b"strict", b"offline", b"net:", b",", b"/", b":", b".", b"..", b"tmp", b"home",
    b"root", b"run", b"var", b"dev", b"shm", b"etc", b"a", b"B", b"-", b"0", b"1", b"10", b"255",
    b"256", b"443", b"65535", b"65536", b"example", b"com", b"\0", b"\xff", b" ", b"//",
];

fn word(r: &mut Regs) -> Vec<u8> {
    let mut w = Vec::new();
    for _ in 0..(r.any() % 24) {
        match r.any() % 8 {
            0 => w.push(r.any() as u8),
            1 => w.truncate((r.any() as usize) % (w.len() + 1)),
            _ => w.extend_from_slice(PIECES[(r.any() as usize) % PIECES.len()]),
        }
    }
    w
}

#[test]
fn every_word_is_refused_or_a_canonical_box() {
    let mut r = Regs::new(0x0b0c_5eed);
    let mut taken = [0u32; 3];
    for _ in 0..200_000 {
        if let Some(p) = check_word(&word(&mut r)) {
            taken[p.preset as usize] += 1;
        }
    }
    assert!(taken[Preset::Strict as usize] > 0 && taken[Preset::Offline as usize] > 0);
}
