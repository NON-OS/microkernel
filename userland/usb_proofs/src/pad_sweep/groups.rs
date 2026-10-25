// NONOS Operating System (AGPL-3.0-or-later)
//! One Input item and the globals and usages before it, as written and as
//! stated (HID 1.11 6.2.2.4 to 6.2.2.8, as recalled, unverified). Its bits
//! follow the bits before it in its report, kept or not. A field is kept
//! inside a gamepad application when the item is data and variable, of
//! 1 to 32 bits, its usage (listed, else from a range on one page, else
//! the last listed) is a button 1 to 32 or a desktop axis 30h to 39h, and
//! fewer than 64 are kept, of at most 64 from one item.

use super::usages::{pick_usage, usage_of, wanted, RANGES};
use super::writer::{F, W};
use crate::rng::Rng;

pub fn group(w: &mut W, r: &mut Rng) {
    let push = r.one_in(4);
    if push {
        w.out.push(0xA4);
        w.stack.push(w.g);
    }
    let page = r.pick(&[1, 1, 9, 9, 0x0C]);
    let (min, max) = r.pick(&RANGES);
    let (size, count) = (r.pick(&[1, 1, 4, 8, 16, 32, 33, 40]), r.pick(&[1, 2, 4, 8, 70]));
    for (tag, v, signed) in [
        (0x04, page as i64, false),
        (0x14, min, true),
        (0x24, max, true),
        (0x74, size, false),
        (0x94, count, false),
    ] {
        w.item(r, tag, v, signed);
    }
    (w.g.page, w.g.min, w.g.max, w.g.size, w.g.count) =
        (page, min as i32, max as i32, size as u32, count as u32);
    let (mut list, mut range) = (vec![], None);
    if r.one_in(3) {
        let (lo, n) = (pick_usage(r, page), r.below(40) as u16);
        let hi_page = if r.one_in(6) { page ^ 8 } else { page };
        w.item(r, 0x18, lo as i64, false);
        w.item(r, 0x28, ((hi_page as i64) << 16) | (lo + n) as i64, false);
        // The maximum in four bytes, its page with it.
        range = Some(((page, lo), (hi_page, lo + n)));
    }
    for _ in 0..r.below(5) {
        let u = pick_usage(r, page);
        let own = r.one_in(5).then(|| r.pick(&[1u16, 9, 0x0C]));
        w.item(r, 0x08, own.map_or(u as i64, |p| (p as i64) << 16 | u as i64), false);
        list.push((own.unwrap_or(page), u));
    }
    let data = r.pick(&[2, 2, 2, 6, 3, 0, 1]);
    w.item(r, 0x80, data, false);
    let id = w.g.id;
    let start = *w.offset(id);
    *w.offset(id) = start.saturating_add(size as u32 * count as u32);
    let variable = data & 1 == 0 && data & 2 != 0;
    if w.in_pad && variable && (1..=32).contains(&size) {
        for i in 0..(count as u32).min(64) {
            let Some(u) = usage_of(&list, range, i) else { break };
            if wanted(u) && w.fields.len() < 64 {
                let f: F =
                    (u.0, u.1, id, start + i * size as u32, size as u32, min as i32, max as i32);
                w.fields.push(f);
            }
        }
    }
    if push {
        w.out.push(0xB4);
        w.g = w.stack.pop().unwrap();
    }
}
