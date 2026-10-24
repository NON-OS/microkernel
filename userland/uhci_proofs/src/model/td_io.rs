// NONOS Operating System (AGPL-3.0-or-later)
//! A TD's OUT bytes read from its buffer, and a TD ended with status bits.

use super::transact::{Outcome, STATUS_NAK};
use super::world::World;
use crate::facts::td::*;

pub(super) fn read(w: &World, buf: u32, n: usize) -> Vec<u8> {
    if n == 0 {
        return vec![];
    }
    let at = (buf - super::world::BUS) as usize;
    w.mem[at..at + n].to_vec()
}

pub(super) fn end(w: &mut World, td: u32, ctrl: u32, bits: u32) -> Outcome {
    w.wr32(td + 4, ctrl & !(STATUS_ACTIVE | STATUS_NAK) | bits);
    Outcome::Halted
}
