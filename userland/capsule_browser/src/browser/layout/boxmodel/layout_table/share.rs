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

use alloc::vec::Vec;

use super::super::table_columns::column_widths;
use super::columns::Col;

/* Share `total` over `n` slots by `weight(i)` (evenly when the weights
 * sum to zero), the rounding remainder to the last slot. */
fn share(n: usize, total: i64, weight: &dyn Fn(usize) -> i64) -> Vec<i64> {
    let sum: i64 = (0..n).map(weight).sum();
    let each = |i: usize| if sum > 0 { total * weight(i) / sum } else { total / n.max(1) as i64 };
    let mut out: Vec<i64> = (0..n).map(each).collect();
    let given: i64 = out.iter().sum();
    if let Some(last) = out.last_mut() {
        *last += total - given;
    }
    out
}

/* Grow `span` so the field `f` sums to at least `need`, the shortfall
 * shared by the columns' max widths. */
pub(super) fn widen(span: &mut [Col], need: i32, f: fn(&mut Col) -> &mut i32) {
    let have: i64 = span.iter_mut().map(|k| *f(k) as i64).sum();
    let short = need as i64 - have;
    if short <= 0 || span.is_empty() {
        return;
    }
    let max: Vec<i64> = span.iter().map(|k| k.max.max(0) as i64).collect();
    for (k, add) in span.iter_mut().zip(share(max.len(), short, &|i| max[i])) {
        *f(k) += add as i32;
    }
}

/* Column widths filling `avail` px (CSS 2.1 17.5.2.2). With room for
 * every maximum, each column takes its maximum and the columns no cell
 * fixed share the rest by their maxima (all columns do when every one
 * is fixed); with less, column_widths shrinks them toward their minima. */
pub(super) fn distribute(cols: &[Col], avail: i32) -> Vec<i32> {
    let (max, min): (Vec<i32>, Vec<i32>) = cols.iter().map(|c| (c.max, c.min)).unzip();
    let hi = sum(cols, |c| c.max);
    if cols.is_empty() || (avail.max(0) as i64) < hi {
        return column_widths(&max, &min, avail);
    }
    let free = cols.iter().any(|c| !c.fixed);
    let w = |i: usize| if free && cols[i].fixed { 0 } else { cols[i].max.max(1) as i64 };
    let extra = share(cols.len(), avail as i64 - hi, &w);
    cols.iter()
        .zip(extra)
        .map(|(c, a)| (c.max as i64 + a).clamp(0, i32::MAX as i64) as i32)
        .collect()
}

fn sum(cols: &[Col], f: fn(&Col) -> i32) -> i64 {
    cols.iter().map(|c| f(c).max(0) as i64).sum()
}
