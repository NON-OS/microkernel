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

//! MCFG's windows: declared lengths that lie, windows cut short or with
//! buses inverted, and a flood past the cap.

use super::gen::{damage, sdt, Rng};
use super::mcfg_decode::{windows, MAX_WINDOWS};
use super::sweep::{held, sweep};

#[test]
fn mcfg_windows_stay_inside_the_table() {
    let make = |r: &mut Rng| {
        let n = 8 + 16 * r.below(9) + r.below(16);
        let mut b = r.bytes(n);
        for w in b[8..].chunks_exact_mut(16) {
            // one window in four keeps whatever buses the bytes say
            let (lo, hi) = (r.below(64) as u8, 64 + r.below(192) as u8);
            if !r.one_in(4) {
                w[10..12].copy_from_slice(&[lo, hi]);
            }
        }
        damage(sdt(b"MCFG", 1, &b), r)
    };
    sweep(
        make,
        |t| Some(windows(t)).filter(|w| !w.is_empty()),
        |w, t| {
            let sound = w.iter().all(|w| w.base != 0 && w.start_bus <= w.end_bus);
            sound && w.len() <= MAX_WINDOWS && 44 + 16 * w.len() <= held(t)
        },
    );
}

#[test]
fn mcfg_stops_at_its_cap() {
    let window = [vec![0, 0, 0, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0xFF], vec![0; 4]].concat();
    let t = sdt(b"MCFG", 1, &[vec![0; 8], window.repeat(MAX_WINDOWS + 7)].concat());
    assert_eq!(windows(&t).len(), MAX_WINDOWS);
}
