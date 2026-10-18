// NONOS Operating System (AGPL-3.0-or-later)
//! W4: a transmit queue whose oldest frame goes unanswered 2.5 s is stuck,
//! as the serving loop's looks show it; an answer that moves the oldest slot
//! starts the count again, and an empty or restarted queue is never stuck.

use std::cell::RefCell;

use crate::gen3::outcome::{Failure, STAGE_FIRMWARE_FAILED, STEP_TX_STUCK};
use crate::gen3::txq::answered::TX_STUCK_MS;
use crate::gen3::txq::{TxQueue, TX_REGION};
use crate::gen3_model::Mem;
use crate::regs::Mmio;

/// Registers that take every write and read zero.
struct Quiet(RefCell<u32>);

impl Mmio for Quiet {
    fn read32(&self, _off: usize) -> u32 {
        0
    }
    fn write32(&self, _off: usize, val: u32) {
        *self.0.borrow_mut() = val;
    }
}

fn frame() -> Vec<u8> {
    let mut f = vec![0xB0, 0x00, 0, 0];
    f.extend_from_slice(&[0xAA; 18]);
    f.extend_from_slice(&[0x10, 0x00, 1, 2, 3]);
    f
}

#[test]
fn an_unanswered_frame_makes_its_queue_stuck_and_an_answer_restarts_the_count() {
    let (tx, m) = (Mem::new(TX_REGION, 0x8000_0000), Quiet(RefCell::new(0)));
    let mut q = TxQueue::new(0);
    q.start(2, 0);
    assert!(!q.stuck(0) && !q.stuck(u64::MAX), "nothing in flight");
    assert!(q.send(&m, &*tx, &frame(), 0) && q.send(&m, &*tx, &frame(), 0));
    assert!(!q.stuck(1_000), "first seen at 1 s");
    assert!(!q.stuck(1_000 + TX_STUCK_MS - 1));
    assert!(q.reclaim(1), "the oldest answered");
    assert!(!q.stuck(1_000 + TX_STUCK_MS), "the next is first seen now");
    assert!(!q.stuck(1_000 + 2 * TX_STUCK_MS - 1));
    assert!(q.stuck(1_000 + 2 * TX_STUCK_MS), "2.5 s with no answer");
    assert!(q.reclaim(2));
    assert!(!q.stuck(u64::MAX), "all answered");
    assert!(q.send(&m, &*tx, &frame(), 0));
    assert!(!q.stuck(5) && q.stuck(5 + TX_STUCK_MS));
    q.start(3, 0);
    assert!(!q.stuck(u64::MAX), "a queue started afresh owes nothing");
    let f = Failure::TxStuck;
    assert_eq!((f.stage(), f.step(), f.detail()), (STAGE_FIRMWARE_FAILED, STEP_TX_STUCK, 0));
    assert_eq!(f.text(), "a transmitted frame went unanswered");
}
