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

//! The seeded sweep: files built from the format's own pieces, cut, doubled
//! and mixed with any byte, each held to `check_file`. Some must be taken,
//! of every base.

use agentproof_policy::Base;

use super::corpus::PIECES;
use super::random::Regs;
use crate::check::check_file;

pub fn text(r: &mut Regs) -> Vec<u8> {
    let mut t = b"nonos-box 1\n".to_vec();
    if r.below(8) == 0 {
        t.clear();
    }
    for _ in 0..r.below(12) {
        match r.below(10) {
            0 => t.push(r.any() as u8),
            1 => t.truncate(r.below(t.len() + 1)),
            _ => t.extend_from_slice(PIECES[r.below(PIECES.len())]),
        }
    }
    t
}

#[test]
fn every_file_is_refused_or_a_canonical_box() {
    let mut r = Regs::new(0x5eed_a9e7);
    let mut taken = [0u32; 3];
    for _ in 0..1_000_000 {
        if let Some(c) = check_file(&text(&mut r)) {
            let base = match c.word.as_slice() {
                b"strict" => Base::Strict,
                b"offline" => Base::Offline,
                _ => Base::Folders,
            };
            taken[base as usize] += 1;
        }
    }
    assert!(taken.iter().all(|n| *n > 0), "taken {taken:?}");
}

#[test]
fn a_byte_flipped_anywhere_is_refused_or_still_canonical() {
    let good = super::file(&["nonos-box 1", "base folders", "write /tmp/out", "network none"]);
    for i in 0..good.len() {
        for bit in 0..8 {
            let mut t = good.clone();
            t[i] ^= 1 << bit;
            check_file(&t);
        }
    }
}
