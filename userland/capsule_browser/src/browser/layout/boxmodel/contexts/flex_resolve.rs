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

use super::flex_item::FlexItem;

/* Resolve the flexible lengths of one line (CSS Flexbox 9.7) with `space`
 * px for the items' margin boxes: free space goes to items that grow, by
 * grow factor; missing space comes from items that shrink, by shrink
 * factor times base size. An item a min or max clamp stops is frozen there
 * and the rest share again, at most once per item, so the loop ends. */
pub(in super::super) fn resolve(items: &mut [FlexItem], space: i32) {
    let margins: i64 = items.iter().map(|it| (it.m[1] + it.m[3]) as i64).sum();
    let hyp: i64 = items.iter().map(|it| it.size as i64).sum();
    let grow = hyp + margins < space as i64;
    let inflexible = |it: &FlexItem| {
        factor(it, grow) == 0 || (grow && it.base > it.size) || (!grow && it.base < it.size)
    };
    let mut frozen: Vec<bool> = items.iter().map(inflexible).collect();
    let spent = |items: &[FlexItem], frozen: &[bool]| -> i64 {
        items.iter().zip(frozen).map(|(it, &f)| if f { it.size } else { it.base } as i64).sum()
    };
    let initial = space as i64 - margins - spent(items, &frozen);
    for _ in 0..=items.len() {
        let free = space as i64 - margins - spent(items, &frozen);
        let sum: i64 =
            items.iter().zip(&frozen).filter(|(_, &f)| !f).map(|(it, _)| factor(it, grow)).sum();
        if sum == 0 {
            break;
        }
        /* Grow factors summing under 1 hand out only that share of the space. */
        let free =
            if grow && sum < 100 * 100 { free.min(initial * sum / (100 * 100)) } else { free };
        let mut violation = 0i64;
        let mut targets: Vec<(usize, i32, i32)> = Vec::new();
        for (i, it) in items.iter().enumerate().filter(|(i, _)| !frozen[*i]) {
            let t = it.base as i64 + free.saturating_mul(factor(it, grow)) / sum;
            let t = t.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
            let c = it.clamp(t);
            violation += c as i64 - t as i64;
            targets.push((i, t, c));
        }
        for &(i, t, c) in &targets {
            items[i].size = c;
            frozen[i] |= violation == 0 || (violation > 0 && c > t) || (violation < 0 && c < t);
        }
        if violation == 0 {
            break;
        }
    }
}

/* The factor an item flexes by: its grow factor, or its shrink factor
 * scaled by its base size (both in hundredths). */
fn factor(it: &FlexItem, grow: bool) -> i64 {
    if grow {
        it.grow * 100
    } else {
        it.shrink * it.base.max(0) as i64
    }
}
