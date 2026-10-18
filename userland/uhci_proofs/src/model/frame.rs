// NONOS Operating System (AGPL-3.0-or-later)
//! One frame as the controller runs it: the frame list entry for FRNUM,
//! then each QH in turn. A queue runs its element TD; a TD done moves the
//! element on to its link, and the next TD runs in the same frame only when
//! the link asks depth first. A NAK leaves the TD active for the next
//! frame; a STALL, an error or a short packet with SPD halts the queue on
//! the TD (its element is not moved).

use super::transact::{transact, Outcome};
use super::world::World;
use crate::facts::td::{LINK_DEPTH, LINK_QH, LINK_TERMINATE, STATUS_ACTIVE};

pub fn run(w: &mut World) {
    let entry = w.rd32(w.flbase + 4 * (w.frnum as u32 & 0x3FF));
    w.frnum = (w.frnum + 1) & 0x7FF;
    let mut link = entry;
    // A bound against a schedule that loops.
    for _ in 0..64 {
        if link & LINK_TERMINATE != 0 || link & LINK_QH == 0 {
            return;
        }
        let qh = link & !0xF;
        queue(w, qh);
        link = w.rd32(qh);
    }
    panic!("the schedule loops");
}

fn queue(w: &mut World, qh: u32) {
    for _ in 0..1024 {
        let elem = w.rd32(qh + 4);
        if elem & (LINK_TERMINATE | LINK_QH) != 0 {
            return;
        }
        let td = elem & !0xF;
        if w.rd32(td + 4) & STATUS_ACTIVE == 0 {
            return;
        }
        match transact(w, td) {
            Outcome::Done => {
                let next = w.rd32(td);
                w.wr32(qh + 4, next);
                if next & LINK_DEPTH == 0 {
                    return;
                }
            }
            Outcome::Halted | Outcome::Nak => return,
        }
    }
}
