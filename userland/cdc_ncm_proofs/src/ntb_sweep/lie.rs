// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in a sound NTB, and the honest datagrams that must still be
//! found after it: none after a header lie; those of the NDPs before a
//! broken one; all but a skipped NDP's; all but an entry and what follows it
//! in its own NDP; the first forty of more.

use super::build::{block, Block};
use super::head_lie::head_lie;
pub use super::lies::{Lie, LIES};
use super::ndp_lie::ndp_lie;
use super::rng::Rng;

pub struct Told {
    pub lie: Lie,
    pub b: Block,
    pub offered32: bool,
    pub expect: Vec<(usize, usize)>,
}

pub fn tell(r: &mut Rng) -> Told {
    loop {
        let lie = LIES[r.range(0, LIES.len() - 1)];
        let mut b = block(r, lie == Lie::TooMany);
        let told = match lie {
            Lie::TooMany => Some((true, keep(&b, &|_, _| true).into_iter().take(40).collect())),
            Lie::NthSign | Lie::NthLen | Lie::Ntb32NotOffered => head_lie(r, lie, &mut b),
            Lie::BlockPastTransfer | Lie::BlockOverRx | Lie::FirstInHeader => {
                head_lie(r, lie, &mut b)
            }
            _ => ndp_lie(r, lie, &mut b),
        };
        if let Some((offered32, expect)) = told {
            return Told { lie, b, offered32, expect };
        }
    }
}

/// The block's datagrams, entry `i` of NDP `k` (chain order) where `f`
/// holds.
pub fn keep(b: &Block, f: &dyn Fn(usize, usize) -> bool) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (k, (_, grams)) in b.ndps.iter().enumerate() {
        out.extend(grams.iter().enumerate().filter(|&(i, _)| f(k, i)).map(|(_, &d)| d));
    }
    out
}
