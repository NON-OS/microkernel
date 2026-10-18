// NONOS Operating System (AGPL-3.0-or-later)
//! Reading the host status: running until idle with an end recorded, and
//! which error is answered when more than one bit shows.

use crate::errno::{E_AGAIN, E_IO, E_NXIO};
use crate::regs::*;
use crate::xfer::status::{seen, Seen};

#[test]
fn busy_or_idle_without_an_end_is_still_running() {
    assert_eq!(seen(STS_HOST_BUSY | STS_INTR), Seen::Running);
    assert_eq!(seen(0), Seen::Running);
    assert_eq!(seen(STS_INUSE | STS_BYTE_DONE), Seen::Running);
}

#[test]
fn intr_alone_is_finished_and_each_error_is_its_own() {
    assert_eq!(seen(STS_INTR | STS_INUSE), Seen::Finished);
    assert_eq!(seen(STS_DEV_ERR), Seen::Fault(E_NXIO));
    assert_eq!(seen(STS_BUS_ERR), Seen::Fault(E_AGAIN));
    assert_eq!(seen(STS_FAILED), Seen::Fault(E_IO));
    assert_eq!(seen(STS_FAILED | STS_INTR), Seen::Fault(E_IO));
}
