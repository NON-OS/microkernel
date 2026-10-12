// NONOS Operating System (AGPL-3.0-or-later)
//! The link when the chip stops answering or will not settle: a PHY that
//! is not read three times running takes the link down, and a link reset
//! that fails is bounded and not run again at once.

use std::time::Duration;

use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::{Nic, Setup};

use crate::bound::bound;
use crate::chip::Chip;

const FIFO: Call = Call::In(Setup::new(0xC0, 0x81, 0x8c, 0), 4);

fn fifo_reads(bus: &MockBus) -> usize {
    bus.0.borrow().calls.iter().filter(|c| **c == FIFO).count()
}

fn after_a_second() {
    std::thread::sleep(Duration::from_millis(1_050));
}

#[test]
fn a_phy_that_stops_answering_takes_the_link_down_after_three_looks() {
    let chip = Chip::default();
    let (_, mut nic) = bound(&chip);
    chip.physr.set(0xa400);
    assert_eq!(nic.tick(), Ok(()));
    assert!(nic.link_up());
    chip.phy_gone.set(true);
    for look in 1..=3 {
        after_a_second();
        assert_eq!(nic.tick(), Err(-71), "the run loop counts it toward the device gone");
        assert_eq!(nic.link_up(), look < 3, "after {look} unanswered looks");
    }
}

#[test]
fn a_link_reset_that_cannot_drain_the_fifo_is_bounded_and_not_rerun_at_once() {
    let chip = Chip { fifo_busy: true, ..Chip::default() };
    let (bus, mut nic) = bound(&chip);
    chip.physr.set(0xa400);
    assert_eq!(nic.tick(), Ok(()), "a FIFO that did not drain is no transfer error");
    let reads = fifo_reads(&bus);
    assert!((2..=10).contains(&reads), "{reads} FIFO reads in one link reset");
    after_a_second();
    assert_eq!(nic.tick(), Ok(()));
    assert_eq!(fifo_reads(&bus), reads, "no second link reset within five seconds");
    assert!(!nic.link_up());
}
