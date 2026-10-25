// NONOS Operating System (AGPL-3.0-or-later)
//! The shared wait over a slot the controller writes mid-read, and the read
//! of a real slot in memory.

use super::ring::{entry, ok_status};
use super::torn_tests::slot;
use crate::admin::{read_entry, read_slot, wait_for_completion, CqCursor};

#[test]
fn the_wait_takes_its_own_entry_when_the_write_lands_mid_read() {
    for flip in 0..=4 {
        let load = slot(entry(3, 1, 0, ok_status(false)), entry(11, 1, 0, ok_status(true)), flip);
        let mut cursor = CqCursor::new(8);
        let mut rung = Vec::new();
        let checks = core::cell::Cell::new(0u32);
        let done = wait_for_completion(
            &mut cursor,
            1,
            11,
            |_| read_entry(|| load().status, || {}, &load),
            |head| rung.push(head),
            || {
                checks.set(checks.get() + 1);
                checks.get() > 4
            },
        );
        assert!(done.is_ok(), "write after {flip} loads: {done:?}");
        assert_eq!(rung, [1], "write after {flip} loads consumed {rung:?}");
    }
}

#[test]
fn a_slot_in_memory_reads_back_whole() {
    for (cid, status) in [(0u16, 0u16), (7, 1), (0xfffe, 0x8003), (0x1234, 0x4321)] {
        let slot = entry(cid, 1, 6, status);
        // SAFETY: a completion entry on this stack, read in place.
        let read = unsafe { read_slot(&slot as *const _ as u64) };
        assert_eq!((read.cid, read.status, read.sq_id, read.sq_head), (cid, status, 1, 6));
    }
}

const IO_RING: &str = include_str!("../../../capsule_driver_nvme/src/nvm/ring_wait.rs");
const ADMIN_RING: &str = include_str!("../../../capsule_driver_nvme/src/admin/queue/wait_ms.rs");

#[test]
fn both_rings_read_their_entries_in_that_order() {
    for ring in [IO_RING, ADMIN_RING] {
        assert!(ring.contains("unsafe { read_slot(slot) }"));
        assert!(!ring.contains("*const Completion"), "a ring reads its entry unordered");
    }
}
