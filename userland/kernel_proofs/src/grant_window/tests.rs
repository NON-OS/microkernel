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

//! A window never overlaps itself, leaves its bounds or runs dry from churn.

use super::va_window::VaWindow;

const BASE: u64 = 0x80_0000_0000;
const P: u64 = 4096;

#[test]
fn a_range_given_back_is_handed_out_again() {
    let mut w = VaWindow::new(BASE, BASE + 8 * P);
    let (a, b) = (w.reserve(2 * P).unwrap(), w.reserve(2 * P).unwrap());
    assert_eq!((a, b), (BASE, BASE + 2 * P));
    w.release(a, 2 * P);
    assert_eq!(w.reserve(P), Some(BASE), "first fit, from what came back");
    assert_eq!(w.reserve(P), Some(BASE + P));
    assert_eq!(w.reserve(4 * P), Some(BASE + 4 * P), "then the top");
    assert_eq!(w.reserve(P), None, "spent");
    assert_eq!(w.reserve(0), None);
}

#[test]
fn neighbours_given_back_join_and_lower_the_top() {
    let mut w = VaWindow::new(BASE, BASE + 4 * P);
    let r: Vec<u64> = (0..4).map(|_| w.reserve(P).unwrap()).collect();
    for i in [2, 0, 1] {
        w.release(r[i], P);
    }
    assert_eq!(w.reserve(3 * P), Some(BASE), "three pages back as one range");
    w.release(BASE, 3 * P);
    w.release(r[3], P);
    assert_eq!(w.reserve(4 * P), Some(BASE), "all of it, after all came back");
}

#[test]
fn many_maps_and_unmaps_never_overlap_or_run_dry() {
    let mut w = VaWindow::new(BASE, BASE + 64 * P);
    let mut held: Vec<(u64, u64)> = Vec::new();
    let mut seed = 7u64;
    for _ in 0..20_000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        if seed >> 63 == 0 && !held.is_empty() {
            let (s, n) = held.swap_remove((seed >> 8) as usize % held.len());
            w.release(s, n);
        } else if let Some(s) = w.reserve(((seed >> 20) % 4 + 1) * P) {
            let n = ((seed >> 20) % 4 + 1) * P;
            assert!(s >= BASE && s + n <= BASE + 64 * P, "inside the window");
            assert!(held.iter().all(|&(t, m)| s + n <= t || t + m <= s), "overlap at {s:#x}");
            held.push((s, n));
        }
    }
    held.drain(..).for_each(|(s, n)| w.release(s, n));
    assert_eq!(w.reserve(64 * P), Some(BASE), "nothing leaked");
}
