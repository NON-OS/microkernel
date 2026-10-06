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

//! Every free cell of the assembled circuit, classified; a cell no rule reads
//! and no reason explains stops the ship.
//!
//! The method of the STARK lane's `cell_audit_test`: change each cell alone and
//! ask the two windows that can see it and the boundaries on it. The counts are
//! pinned as a ratchet, so a new free cell fails here until it has a reason.
//!
//! Measured at 2^14 rows: 540,672 cells, 58,668 read, 482,004 free, none inside
//! a live chain.

use std::collections::BTreeMap;

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::field::Fp;

use super::satisfy::{honest, runs, violation};

/// The padding region's kind: the fifth, after the four chains.
const PAD_KIND: usize = 4;

#[test]
fn the_honest_witness_satisfies() {
    let (b, t) = honest();
    assert_eq!(violation(&b.wired, &t), None);
}

/// Why a free cell is free, or `None` when nothing explains it.
fn rule_for(
    owner: Option<(usize, usize, usize)>,
    r: usize,
    c: usize,
    stack_w: usize,
    width: usize,
) -> Option<&'static str> {
    let Some((_, e, k)) = owner else {
        return Some("a: padding row");
    };
    if k == PAD_KIND {
        return Some("a: padding region");
    }
    if c >= stack_w {
        return Some("a: mask column");
    }
    if c >= width {
        return Some("a: past the region's width");
    }
    if r == e + 1 {
        return Some("b: region's last row");
    }
    None
}

#[test]
fn every_free_cell_in_the_trace_has_a_rule() {
    let (b, t) = honest();
    let air = &b.wired;
    let wm = air.wired();
    let (w, ws) = (air.trace_width(), air.window_size());
    let n = 1usize << air.log_trace_len();
    let map = wm.kind_map();
    let stack_w = w - wm.product_columns() - wm.mask_columns();
    let runs = runs(air, map.len());
    assert_eq!(runs.len(), 5, "one run per region");
    assert_eq!(air.log_trace_len(), 14, "the certified length");
    let owner = |r: usize| runs.iter().find(|(s, e, _)| *s <= r && r <= *e + 1).copied();

    let per = air.periodic_columns();
    let bnd: BTreeMap<(usize, usize), Fp> =
        air.boundary().into_iter().map(|(c, r, v)| ((r, c), v)).collect();
    let holds = |t: &[Fp], r: usize| -> bool {
        if r + ws > n {
            return true;
        }
        let p: Vec<Fp> = per.iter().map(|c| c[r]).collect();
        air.transition(&t[r * w..(r + ws) * w], &p).iter().all(|v| *v == Fp::ZERO)
    };
    let mut by_rule: BTreeMap<&str, usize> = BTreeMap::new();
    let mut unexplained = Vec::new();
    let mut total_free = 0usize;
    let mut tt = t.clone();
    for r in 0..n {
        for c in 0..w {
            let old = tt[r * w + c];
            tt[r * w + c] = old + Fp::ONE;
            let seen = (r > 0 && !holds(&tt, r - 1))
                || !holds(&tt, r)
                || bnd.get(&(r, c)).is_some_and(|v| tt[r * w + c] != *v);
            tt[r * w + c] = old;
            if seen {
                continue;
            }
            total_free += 1;
            let o = owner(r);
            match rule_for(o, r, c, stack_w, o.map(|(_, _, k)| map[k].4).unwrap_or(0)) {
                Some(why) => *by_rule.entry(why).or_insert(0) += 1,
                None => unexplained.push((r, c, o.map(|(_, _, k)| k))),
            }
        }
    }
    println!("cells {}, read {}, free {total_free}", n * w, n * w - total_free);
    for (why, count) in &by_rule {
        println!("  {why}: {count}");
    }
    assert!(
        unexplained.is_empty(),
        "free cells no rule explains: {:?}",
        &unexplained[..unexplained.len().min(40)]
    );
    /*
     * The ratchet. No free cell sits inside a live chain: every one is a mask
     * column, the padding region that brings the trace to the pool's length, a
     * padding row below the stack, or a chain's last row, which is read only as
     * its last live row's successor.
     */
    assert_eq!(n * w, 540_672);
    assert_eq!(n * w - total_free, 58_668);
    assert_eq!(by_rule.get("a: mask column"), Some(&1_792));
    assert_eq!(by_rule.get("a: padding region"), Some(&253_952));
    assert_eq!(by_rule.get("a: padding row"), Some(&226_176));
    assert_eq!(by_rule.get("b: region's last row"), Some(&84));
    assert_eq!(by_rule.len(), 4);
}
