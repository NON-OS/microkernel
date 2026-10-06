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

//! The checkpoint rule in all four regions: the row a region's last
//! compression writes, whose rate lanes the root or tag pin reads.
//!
//! The STARK lane's `checkpoint_test`, over this circuit. Every cell the
//! per-row rules read on a checkpoint row and its successor, changed alone, is
//! refused. And a checkpoint made to show a digest its walk never reached is
//! refused in every region: a wrong sibling below it with the true root shown,
//! or, in the single-compression tag region, a chosen digest.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::field::Fp;

use super::satisfy::{honest, violation};

const WIDTH: usize = 8;
const RATE: usize = 4;

/// Every (kind, row) the rule treats as a checkpoint: the kind's own
/// checkpoint selector, its last periodic slot. The launch rule has no such
/// slot; there it is the slot boundary whose successor the honest walk leaves
/// with an empty capacity.
fn checkpoint_rows(
    air: &impl Air,
    map: &[(usize, usize, usize, usize, usize)],
    t: &[Fp],
) -> Vec<(usize, usize)> {
    let cols = air.periodic_columns();
    let w = air.trace_width();
    let mut out = Vec::new();
    // The four chains; the padding region has no checkpoint.
    for (k, &(first, slots, ..)) in map.iter().enumerate().take(4) {
        for r in 0..cols[k].len() - 1 {
            if cols[k][r] != Fp::ONE {
                continue;
            }
            let hit = if cfg!(feature = "launch_v1") {
                cols[first + WIDTH][r] == Fp::ONE
                    && (RATE..WIDTH).all(|j| t[(r + 1) * w + j] == Fp::ZERO)
            } else {
                cols[first + slots - 1][r] == Fp::ONE
            };
            if hit {
                out.push((k, r));
            }
        }
    }
    out
}

/// Re-fill a region from `from` to its last live row so every transition holds
/// after a cell was changed: the squares from each row's state, then the next
/// row's state from the transition's residual.
fn refill(air: &impl Air, t: &mut [Fp], per: &[Vec<Fp>], from: usize, last_live: usize) {
    let w = air.trace_width();
    let sq = WIDTH + 1 + RATE;
    for r in from..=last_live + 1 {
        for j in 0..WIDTH {
            let x2 = t[r * w + j] * t[r * w + j];
            t[r * w + sq + j] = x2;
            t[r * w + sq + WIDTH + j] = x2 * x2;
        }
        if r > last_live {
            break;
        }
        let p: Vec<Fp> = per.iter().map(|c| c[r]).collect();
        let out = air.transition(&t[r * w..(r + 2) * w], &p);
        for j in 0..WIDTH {
            t[(r + 1) * w + j] = t[(r + 1) * w + j] - out[j];
        }
    }
}

#[test]
fn every_region_has_one_checkpoint() {
    let (b, t) = honest();
    let map = b.wired.wired().kind_map();
    let rows = checkpoint_rows(&b.wired, &map, &t);
    let kinds: Vec<usize> = rows.iter().map(|&(k, _)| k).collect();
    assert_eq!(kinds, vec![0, 1, 2, 3], "one checkpoint per region, in kind order: {rows:?}");
}

#[test]
fn every_checkpoint_cell_changed_alone_is_refused() {
    let (b, t) = honest();
    let air = &b.wired;
    let w = air.trace_width();
    let map = air.wired().kind_map();
    let per_row = (WIDTH + 1 + RATE + 2 * WIDTH).min(w);
    let mut free = Vec::new();
    let mut checked = 0usize;
    for (k, r) in checkpoint_rows(air, &map, &t) {
        for row in [r, r + 1] {
            for c in 0..per_row {
                let mut x = t.clone();
                x[row * w + c] =
                    if c == WIDTH { Fp::ONE - x[row * w + c] } else { x[row * w + c] + Fp::ONE };
                if violation(air, &x).is_none() {
                    free.push((k, row, c));
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 4 * 2 * per_row);
    #[cfg(not(feature = "launch_v1"))]
    assert!(free.is_empty(), "checkpoint cells left free: {free:?}");
    #[cfg(feature = "launch_v1")]
    assert!(!free.is_empty(), "the launch rule leaves the checkpoint direction unread");
}

#[test]
fn a_checkpoint_showing_an_unwalked_digest_is_refused_in_every_region() {
    let (b, t0) = honest();
    let air = &b.wired;
    let per = air.periodic_columns();
    let map = air.wired().kind_map();
    let w = air.trace_width();
    let mut accepted = Vec::new();
    let mut below = 0usize;
    for (k, r) in checkpoint_rows(air, &map, &t0) {
        let (first, ..) = map[k];
        let slot = first + WIDTH;
        let op = first + WIDTH + 1;
        let mut last_live = r;
        while per[k][last_live + 1] == Fp::ONE {
            last_live += 1;
        }
        let mut s = r;
        let srow = loop {
            if s == 0 || per[k][s - 1] != Fp::ONE || per[op][s - 1] == Fp::ONE {
                break None;
            }
            s -= 1;
            if per[slot][s] == Fp::ONE {
                break Some(s);
            }
        };
        let mut t = t0.clone();
        let digest: Vec<Fp> = t[(r + 1) * w..(r + 1) * w + RATE].to_vec();
        let shown: Vec<Fp> = match srow {
            Some(srow) => {
                below += 1;
                t[srow * w + WIDTH + 1] = t[srow * w + WIDTH + 1] + Fp::ONE;
                refill(air, &mut t, &per, srow, last_live);
                assert_ne!(
                    t[(r + 1) * w..(r + 1) * w + RATE],
                    digest[..],
                    "the wrong sibling moved nothing"
                );
                digest
            }
            None => digest.iter().map(|d| *d + Fp::ONE).collect(),
        };
        t[r * w + WIDTH] = Fp::ONE;
        for c in 0..RATE {
            t[r * w + WIDTH + 1 + c] = shown[c];
        }
        refill(air, &mut t, &per, r, last_live);
        if violation(air, &t).is_none() {
            accepted.push((k, r));
        }
    }
    assert_eq!(
        below, 3,
        "the bootloader, kernel and device regions have a sibling below their checkpoint"
    );
    #[cfg(not(feature = "launch_v1"))]
    assert!(accepted.is_empty(), "unwalked digests accepted at {accepted:?}");
    #[cfg(feature = "launch_v1")]
    assert_eq!(
        accepted.len(),
        below,
        "the launch rule accepts every wrong path below a checkpoint"
    );
}
