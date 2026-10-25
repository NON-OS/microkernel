// NONOS Operating System (AGPL-3.0-or-later)
//! A QH visited: a halted one is skipped; an overlay with no active qTD
//! fetches the one its next pointer names (none: nothing to do); the
//! current qTD then runs (packets.rs), and a retired one lets the next be
//! fetched in the same visit.

use super::packets::packets;
use super::world::World;
use crate::facts::qh::{LINK_T, QH_ALT, QH_CURRENT, QH_NEXT, QH_TOKEN};
use crate::facts::qtd::{TOK_ACTIVE, TOK_DT, TOK_HALTED};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pass {
    Periodic,
    Async,
}

pub fn run_qh(w: &mut World, qh: u32, pass: Pass) {
    if pass == Pass::Periodic && w.rd32(qh + 8) & 1 == 0 {
        return;
    }
    for _ in 0..8 {
        let tok = w.rd32(qh + QH_TOKEN as u32);
        if tok & TOK_HALTED != 0 {
            return;
        }
        let q = if tok & TOK_ACTIVE != 0 {
            w.rd32(qh + QH_CURRENT as u32)
        } else {
            let next = w.rd32(qh + QH_NEXT as u32);
            if next & LINK_T != 0 || w.rd32(next + 8) & TOK_ACTIVE == 0 {
                return;
            }
            w.wr32(qh + QH_CURRENT as u32, next);
            w.wr32(qh + QH_NEXT as u32, w.rd32(next));
            w.wr32(qh + QH_ALT as u32, w.rd32(next + 4));
            w.wr32(qh + QH_TOKEN as u32, tok & TOK_DT | TOK_ACTIVE);
            next
        };
        if !packets(w, qh, q, pass == Pass::Periodic) {
            return;
        }
    }
}
