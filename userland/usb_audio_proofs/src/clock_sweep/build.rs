// NONOS Operating System (AGPL-3.0-or-later)
//! A 2.0 function's clocks as stated (2.0 4.7.2, as recalled, unverified):
//! one to three Clock Sources, up to two Clock Selectors of one to three
//! pins, the first pin a source, and one to three terminals, input or
//! output, each naming a source or a selector; every id distinct and not
//! 0. A terminal runs on the source it names, or on the first source of
//! the selector it names, which the host selects.

use super::rng::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ent {
    Source {
        id: u8,
        controls: u8,
    },
    Selector {
        id: u8,
        pins: Vec<u8>,
    },
    /// Input (17 bytes) or output (12) terminal and the clock it names.
    Terminal {
        id: u8,
        input: bool,
        clock: u8,
    },
}

pub struct Func {
    pub ac: u8,
    pub ents: Vec<Ent>,
}

fn take(pool: &mut Vec<u8>, r: &mut Rng) -> u8 {
    pool.swap_remove(r.range(0, pool.len() - 1))
}

pub fn func(r: &mut Rng) -> Func {
    let mut pool: Vec<u8> = (1..=60).collect();
    let sources: Vec<u8> = (0..r.range(1, 3)).map(|_| take(&mut pool, r)).collect();
    let mut ents: Vec<Ent> = sources
        .iter()
        .map(|&id| Ent::Source { id, controls: [1, 3, 7, 5][r.range(0, 3)] })
        .collect();
    let mut clocks = sources.clone();
    for _ in 0..r.range(0, 2) {
        let n = r.range(1, 3);
        let pins = (0..n).map(|_| sources[r.range(0, sources.len() - 1)]).collect();
        let s = take(&mut pool, r);
        clocks.push(s);
        ents.push(Ent::Selector { id: s, pins });
    }
    for _ in 0..r.range(1, 3) {
        let (t, clock) = (take(&mut pool, r), clocks[r.range(0, clocks.len() - 1)]);
        ents.push(Ent::Terminal { id: t, input: r.range(0, 1) == 0, clock });
    }
    // Entities come in any order within the interface.
    for i in (1..ents.len()).rev() {
        ents.swap(i, r.range(0, i));
    }
    Func { ac: r.range(0, 3) as u8, ents }
}
