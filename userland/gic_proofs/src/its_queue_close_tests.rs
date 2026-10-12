// NONOS Operating System (AGPL-3.0-or-later)
//! A batch that does not finish closes the ITS command queue, and without
//! that the next batch would write over commands the ITS has not read.

use crate::gic::its::ring::Ring;
use crate::gic::its::ring_types::{Progress, Refused, BYTES, SLOT};
use crate::its_driver::run;
use crate::its_model::ModelIts;

#[test]
fn a_batch_that_times_out_closes_the_queue() {
    let (mut ring, mut its) = (Ring::default(), ModelIts::new());
    let ids: Vec<u64> = (0..100).collect();
    assert_eq!(run(&mut ring, &mut its, &ids, 3, 10), Err(Progress::Pending));
    assert_eq!(ring.place(1), Err(Refused::Closed));
    its.step(usize::MAX);
    assert_eq!((its.consumed, its.overwrites), (ids, 0));
}

#[test]
fn a_stalled_its_closes_the_queue() {
    let (mut ring, mut its) = (Ring::default(), ModelIts::new());
    its.poison = Some(5);
    assert_eq!(run(&mut ring, &mut its, &[1, 2, 3, 4, 5, 6, 7], 100, 2), Err(Progress::Stalled));
    assert_eq!(its.consumed, [1, 2, 3, 4]);
    assert_eq!(ring.place(1), Err(Refused::Closed));
    // Stalled outranks an offset that happens to match.
    assert_eq!(ring.progress(ring.write() | 1), Progress::Stalled);
}

#[test]
fn without_closing_the_next_batch_writes_over_unread_commands() {
    // The hazard `close` is for: a batch the ITS has not finished, then
    // another placed as though the queue were empty.
    let (mut ring, mut its) = (Ring::default(), ModelIts::new());
    let first: Vec<u64> = (0..100).collect();
    let start = ring.place(first.len()).unwrap();
    for (n, id) in first.iter().enumerate() {
        its.store((start + n as u64 * SLOT) % BYTES, *id);
    }
    its.post(ring.write());
    its.step(10);
    let start = ring.place(100).unwrap();
    for n in 0..100u64 {
        its.store((start + n * SLOT) % BYTES, 1000 + n);
    }
    assert!(its.overwrites > 0);
}
