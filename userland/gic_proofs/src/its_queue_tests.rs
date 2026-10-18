// NONOS Operating System (AGPL-3.0-or-later)
//! The ITS command queue against a model ITS: every command read once, in
//! order, across thousands of batches and every wrap, with none written over
//! one still unread; and the batch limit.

use crate::gic::its::ring::Ring;
use crate::gic::its::ring_types::{Refused, BYTES, MAX_BATCH, SLOT};
use crate::its_driver::run;
use crate::its_model::ModelIts;

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn every_command_is_read_once_in_order_across_every_wrap() {
    let (mut ring, mut its, mut rng, mut next) = (Ring::default(), ModelIts::new(), 0x9e37u64, 0);
    let mut sent = Vec::new();
    for _ in 0..5000 {
        let len = 1 + (xorshift(&mut rng) % MAX_BATCH as u64) as usize;
        let ids: Vec<u64> = (next..next + len as u64).collect();
        next += len as u64;
        let chunk = 1 + (xorshift(&mut rng) % 40) as usize;
        assert_eq!(run(&mut ring, &mut its, &ids, 1000, chunk), Ok(()));
        sent.extend(ids);
    }
    assert_eq!((its.overwrites, its.consumed == sent), (0, true));
}

#[test]
fn a_batch_is_one_slot_short_of_the_queue() {
    let mut ring = Ring::default();
    assert_eq!(ring.place(0), Err(Refused::TooLong));
    assert_eq!(ring.place(MAX_BATCH + 1), Err(Refused::TooLong));
    assert_eq!(MAX_BATCH as u64, BYTES / SLOT - 1);
    assert_eq!(ring.place(MAX_BATCH), Ok(0));
    assert_eq!(ring.write(), BYTES - SLOT);
}
