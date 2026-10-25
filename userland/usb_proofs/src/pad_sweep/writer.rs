// NONOS Operating System (AGPL-3.0-or-later)
//! A report descriptor written an item at a time (HID 1.11 6.2.2, as
//! recalled, unverified), and what it states as it is written: the global
//! state with Push and Pop, where each Input's bits fall in its report,
//! the collections, and the gamepad fields kept. A value is written in 0,
//! 1, 2 or 4 bytes, the fewest that hold it, or a byte wider at times.

use crate::rng::Rng;

/// page, usage, report id, bit offset, bit size, logical min, max
pub type F = (u16, u16, u8, u32, u32, i32, i32);

#[derive(Clone, Copy, Default)]
pub struct G {
    pub page: u16,
    pub min: i32,
    pub max: i32,
    pub size: u32,
    pub count: u32,
    pub id: u8,
}

#[derive(Default)]
pub struct W {
    pub out: Vec<u8>,
    pub g: G,
    pub stack: Vec<G>,
    pub offsets: Vec<(u8, u32)>,
    pub fields: Vec<F>,
    pub ids: bool,
    /// A lie that makes it no gamepad at all.
    pub broken: bool,
    pub depth: u32,
    pub in_pad: bool,
    pub seen: bool,
}

impl W {
    /// An item of `tag` (type and tag bits) and `value`, read signed when
    /// `signed` says.
    pub fn item(&mut self, r: &mut Rng, tag: u8, value: i64, signed: bool) {
        let fits = |n: usize| match (n, signed) {
            (0, _) => value == 0,
            (4, _) => true,
            (n, true) => (-(1i64 << (8 * n - 1))..(1i64 << (8 * n - 1))).contains(&value),
            (n, false) => (0..(1i64 << (8 * n))).contains(&value),
        };
        let mut n = [0, 1, 2, 4].into_iter().find(|&n| fits(n)).unwrap();
        // Wider at times, never to four bytes, where a usage carries its
        // own page.
        if n < 2 && r.one_in(5) {
            n += 1;
        }
        self.out.push(tag | [0, 1, 2, 0, 3][n]);
        self.out.extend(&(value as u32).to_le_bytes()[..n]);
        if r.one_in(9) {
            // A long item, which the walk steps over.
            let len = r.below(6) as u8;
            self.out.extend([0xFE, len, 0xF0]);
            self.out.extend(r.bytes(len as usize));
        }
    }

    pub fn offset(&mut self, id: u8) -> &mut u32 {
        if let Some(i) = self.offsets.iter().position(|o| o.0 == id) {
            return &mut self.offsets[i].1;
        }
        self.offsets.push((id, 0));
        &mut self.offsets.last_mut().unwrap().1
    }
}
