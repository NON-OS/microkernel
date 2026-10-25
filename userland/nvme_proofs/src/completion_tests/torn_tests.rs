// NONOS Operating System (AGPL-3.0-or-later)
//! The controller writes an entry while the driver reads it. The phase tag
//! is read first, so wherever the write lands the read sees the slot's last
//! pass (empty) or this pass whole (this command's), never this pass's phase
//! tag with last pass's command id: that read was taken for another
//! command's entry, consumed, and the wait ran out its whole budget.

use core::cell::{Cell, RefCell};

use super::ring::{entry, ok_status};
use crate::admin::{classify, read_entry, Completion, CqStep};

/// A slot holding `old` for its first `flip` loads and `new` after them.
pub(super) fn slot(old: Completion, new: Completion, flip: u32) -> impl Fn() -> Completion {
    let loads = Cell::new(0u32);
    move || {
        let n = loads.get();
        loads.set(n + 1);
        if n < flip { old } else { new }
    }
}

fn torn(old: Completion, new: Completion, flip: u32) -> Completion {
    let load = slot(old, new, flip);
    read_entry(|| load().status, || {}, &load)
}

#[test]
fn a_write_landing_between_the_loads_never_reads_as_another_command() {
    for phase in [false, true] {
        let (old, new) = (entry(3, 1, 0, ok_status(!phase)), entry(11, 1, 0, ok_status(phase)));
        for flip in 0..=2 {
            match classify(torn(old, new, flip), phase, 1, 11) {
                CqStep::Done(Ok(())) => assert_eq!(flip, 0, "done before the write"),
                CqStep::Empty => assert!(flip > 0, "a whole entry read as empty"),
                other => panic!("write after {flip} loads read as {other:?}"),
            }
        }
    }
}

#[test]
fn the_barrier_comes_between_the_status_and_the_rest() {
    let log = RefCell::new(Vec::new());
    read_entry(
        || {
            log.borrow_mut().push("status");
            0
        },
        || log.borrow_mut().push("barrier"),
        || {
            log.borrow_mut().push("rest");
            entry(5, 1, 0, 0)
        },
    );
    assert_eq!(*log.borrow(), ["status", "barrier", "rest"]);
}

#[test]
fn the_phase_tag_is_the_first_reads() {
    let read = read_entry(|| ok_status(true), || {}, || entry(5, 1, 2, ok_status(false)));
    assert!(read.phase());
    assert_eq!((read.cid, read.sq_id, read.sq_head), (5, 1, 2));
}
