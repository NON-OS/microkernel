// NONOS Operating System (AGPL-3.0-or-later)
//! _PSS and _TSS under lying shapes. The model (ACPI 6.5, 8.4.5.2 and
//! 8.4.6.2; recalled, unverified): states are read in order, at most 32
//! P-states and 16 T-states, up to the first that is not a package of six
//! (five) integers each within 32 bits, or, for _TSS, a percentage from 1
//! to 100 no higher than the state before. _PPC and _TPC count states by
//! position, so a state is never skipped: the list stops at the lie.

use super::gen::{lie, Rng};
use super::run::{elements, hold, some, word};
use crate::cpu::{parse_pss, PState};
use crate::object::Object;

pub(super) fn words(o: &Object, n: usize) -> Option<Vec<u32>> {
    let e = elements(o)?;
    (0..n).map(|i| word(&e, i)).collect()
}

fn pss(o: &Object) -> Option<Vec<PState>> {
    let states = elements(o)?.into_iter().take(32).map_while(|s| words(&s, 6));
    some(
        states
            .map(|w| PState {
                mhz: w[0],
                milliwatts: w[1],
                latency_us: w[2],
                bus_master_latency_us: w[3],
                control: w[4],
                status: w[5],
            })
            .collect(),
    )
}

/// Mostly a handful of states; now and then more than any bound allows.
pub(super) fn states(r: &mut Rng, width: usize, first: &mut dyn FnMut(&mut Rng) -> u64) -> Object {
    let n = if r.below(10) == 0 { 40 } else { r.below(6) };
    let mut e = Vec::new();
    for _ in 0..n {
        let head = first(r);
        // A long list is honest state by state, so only the bound ends it.
        let int = |r: &mut Rng, v: u64| if n > 32 { Object::Integer(v) } else { r.int(v) };
        let mut s = vec![int(r, head)];
        s.extend((1..width).map(|i| int(r, i as u64 * 100)));
        e.push(if n > 32 { Object::package(s) } else { lie(r, s) });
    }
    lie(r, e)
}

#[test]
fn pss_is_its_model() {
    let make = |r: &mut Rng| states(r, 6, &mut |r| 3000 - r.below(2000));
    hold("_PSS", 0x955, make, |o| some(parse_pss(o)), pss);
}

#[path = "cst_tests.rs"]
mod cst;

#[path = "tss_tests.rs"]
mod tss;
