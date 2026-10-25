// NONOS Operating System (AGPL-3.0-or-later)
//! A qTD's end as the controller writes it back: retired, halted, or left
//! active where a NAK stopped it; the QH overlay kept in step.

use super::world::World;
use crate::facts::qh::{LINK_T, QH_NEXT, QH_TOKEN};
use crate::facts::qtd::{TOK_ACTIVE, TOK_DT, TOK_HALTED};

fn token(left: usize, dt: bool, rest: u32) -> u32 {
    rest & !(0x7FFF << 16 | TOK_DT) | (left as u32) << 16 | if dt { TOK_DT } else { 0 }
}

/// Still active after a NAK: its bytes left and pointer written back.
pub fn stay(w: &mut World, qh: u32, q: u32, left: usize, ptr: u32, dt: bool, dtc: bool) -> bool {
    let tok = w.rd32(q + 8);
    w.wr32(q + 8, token(left, if dtc { dt } else { tok & TOK_DT != 0 }, tok));
    w.wr32(q + 12, ptr);
    if !dtc {
        let o = w.rd32(qh + QH_TOKEN as u32);
        w.wr32(qh + QH_TOKEN as u32, token(left, dt, o) | TOK_ACTIVE);
    }
    false
}

/// Retired: not active, its bytes left; the overlay inactive with the
/// toggle, and on to the alternate after a short packet when one is named.
pub fn done(w: &mut World, qh: u32, q: u32, left: usize, dt: bool, dtc: bool, short: bool) -> bool {
    let tok = w.rd32(q + 8);
    let qdt = if dtc { dt } else { tok & TOK_DT != 0 };
    w.wr32(q + 8, token(left, qdt, tok) & !TOK_ACTIVE);
    let odt = if dtc { w.rd32(qh + QH_TOKEN as u32) & TOK_DT != 0 } else { dt };
    w.wr32(qh + QH_TOKEN as u32, token(0, odt, 0));
    let alt = w.rd32(q + 4);
    if short && alt & LINK_T == 0 {
        w.wr32(qh + QH_NEXT as u32, alt);
    }
    true
}

/// Halted, with `bits` beside the halt (none: a STALL).
pub fn halt(w: &mut World, qh: u32, q: u32, bits: u32) -> bool {
    let tok = w.rd32(q + 8);
    w.wr32(q + 8, tok & !TOK_ACTIVE | TOK_HALTED | bits);
    let o = w.rd32(qh + QH_TOKEN as u32);
    w.wr32(qh + QH_TOKEN as u32, o & !TOK_ACTIVE | TOK_HALTED | bits);
    false
}
