// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in a run of sound batches, and what it must cost: the
//! bytes lost and counted, and the bytes the host never sees. Every byte
//! sent is kept, counted lost, or unseen.

use super::build::{reply, Rig};
use super::lie_batch::{told, Cost};
pub use super::lies::{Lie, LIES};
use super::rng::Rng;
use crate::ring::RING_CAP;

pub enum Step {
    Batch(Vec<u8>),
    Take(usize),
}

pub struct Told {
    pub rig: Rig,
    pub steps: Vec<Step>,
    /// Bytes the lie must cost; none when a flood decides it.
    pub lost: Option<u32>,
    pub unseen: usize,
    pub sent: usize,
}

pub fn tell(r: &mut Rng, lie: Lie) -> Told {
    let mut rig = Rig::new(r);
    let n = if lie == Lie::Flood { RING_CAP / 2000 + r.range(1, 40) } else { r.range(1, 6) };
    let (j, mut steps, mut sent) = (r.range(0, n - 1), vec![], 0);
    let mut c = Cost { lost: 0, unseen: 0 };
    for b in 0..n {
        let mut ps = rig.batch(r);
        let d = if b == j {
            told(lie, r, &mut rig, &mut ps, &mut c)
        } else {
            reply(&ps, ps.len() as u8)
        };
        sent += ps.iter().map(Vec::len).sum::<usize>();
        steps.push(Step::Batch(d));
        if lie != Lie::Flood && r.range(0, 1) == 0 {
            steps.push(Step::Take([usize::MAX, r.range(0, 5000)][r.range(0, 1)]));
        }
    }
    steps.push(Step::Take(usize::MAX));
    let lost = (lie != Lie::Flood).then_some(c.lost);
    Told { rig, steps, lost, unseen: c.unseen, sent }
}
