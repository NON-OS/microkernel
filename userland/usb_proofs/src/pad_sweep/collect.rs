// NONOS Operating System (AGPL-3.0-or-later)
//! The lies that make a descriptor no gamepad, and its collections: an
//! application at the top opens a gamepad when its usage says one, and
//! the gamepad ends with the collection that opened it.

use super::writer::W;
use crate::rng::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lie {
    Sound,
    PushDeep,
    PopEmpty,
    BadId,
    Usages,
}

pub const LIES: [Lie; 5] = [Lie::Sound, Lie::PushDeep, Lie::PopEmpty, Lie::BadId, Lie::Usages];

pub fn collection(w: &mut W, r: &mut Rng, kind: i64, pad: bool) {
    w.item(r, 0xA0, kind, false);
    if kind == 1 && w.depth == 0 {
        w.in_pad = pad;
        w.seen |= pad;
    }
    w.depth += 1;
}

pub fn end(w: &mut W) {
    w.out.push(0xC0);
    w.depth = w.depth.saturating_sub(1);
    w.in_pad &= w.depth > 0;
}
