// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's side of a batch, step for step as `Queue::run` takes it,
//! with the model ITS in place of the registers and the queue's memory.

use crate::gic::its::ring::Ring;
use crate::gic::its::ring_types::{Progress, BYTES, SLOT};
use crate::its_model::ModelIts;

/// One batch as `Queue::run` does it: place, write, post, poll.
pub fn run(
    ring: &mut Ring,
    its: &mut ModelIts,
    ids: &[u64],
    polls: usize,
    per_poll: usize,
) -> Result<(), Progress> {
    let start = ring.place(ids.len()).map_err(|_| Progress::Pending)?;
    for (n, id) in ids.iter().enumerate() {
        its.store((start + n as u64 * SLOT) % BYTES, *id);
    }
    its.post(ring.write());
    for _ in 0..polls {
        its.step(per_poll);
        match ring.progress(its.creadr()) {
            Progress::Done => return Ok(()),
            Progress::Stalled => break,
            Progress::Pending => {}
        }
    }
    ring.close();
    Err(ring.progress(its.creadr()))
}
