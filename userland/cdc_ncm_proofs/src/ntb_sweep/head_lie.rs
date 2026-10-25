// NONOS Operating System (AGPL-3.0-or-later)
//! A lie in the block's header: after any of them nothing is taken.

use super::build::Block;
use super::lies::Lie;
use super::rng::Rng;

pub fn head_lie(r: &mut Rng, lie: Lie, b: &mut Block) -> Option<(bool, Vec<(usize, usize)>)> {
    let s = b.s;
    match lie {
        Lie::NthSign => b.raw[r.range(0, 3)] ^= 0x20,
        Lie::NthLen => b.put(4, r.one_of(&[0, s.nth_len - 4, s.nth_len + 4, 0xFFFF]) as u64, 2),
        Lie::Ntb32NotOffered if s.wide => return Some((false, Vec::new())),
        Lie::BlockPastTransfer => b.field(8, (b.raw.len() + r.range(1, 64)) as u64),
        Lie::BlockOverRx => b.rx_max = r.range(1, b.raw.len() - 1),
        Lie::FirstInHeader => {
            let at = if s.wide { 12 } else { 10 };
            b.field(at, r.range(1, s.nth_len - 1) as u64)
        }
        _ => return None,
    }
    Some((true, Vec::new()))
}
