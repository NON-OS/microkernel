// NONOS Operating System (AGPL-3.0-or-later)
//! The link from the PHY: down from bind, up with the model's writes once
//! the PHY reports one, the medium following speed and duplex, and the
//! receiver restart bounded however long the FIFO reads busy.

use nonos_usbnet::mock::MockBus;
use nonos_usbnet::Nic;

use crate::ax::follow::RESTARTS;
use crate::ax::link::Ax;
use crate::ax::watch::{DOWN_EVERY, LOOK_EVERY};
use crate::chip::{healthy, Chip};
use crate::harness::{bound, bound_with};
use crate::model::{link_up, Want};
use crate::seen::{phy_reads, vendor};

pub fn ticks(ax: &mut Ax<MockBus>, n: u32) -> Vec<Result<(), i32>> {
    (0..n).map(|_| ax.tick()).collect()
}

#[test]
fn the_link_is_down_until_the_phy_reports_one_then_set_as_the_model_says() {
    let (bus, mut ax, chip) = bound();
    ticks(&mut ax, DOWN_EVERY);
    assert_eq!((ax.link_up(), vendor(&bus)), (false, vec![Want::In(2, 3, 0x11, 2)]));
    chip.borrow_mut().phy = 0xA400;
    bus.0.borrow_mut().calls.clear();
    ticks(&mut ax, DOWN_EVERY);
    let mut want = vec![Want::In(2, 3, 0x11, 2)];
    want.extend(link_up(0x013B));
    assert_eq!((ax.link_up(), vendor(&bus)), (true, want));
}

#[test]
fn the_medium_follows_speed_and_duplex() {
    let cases = [
        (0xA400, 0x013B), // gigabit, full
        (0x8400, 0x0139), // gigabit, half
        (0x6400, 0x0332), // 100, full
        (0x4400, 0x0330), // 100, half
        (0x2400, 0x0132), // 10, full
        (0x0400, 0x0130), // 10, half
        (0xE400, 0x0132), // speed 11b, taken as 10
    ];
    for (phy, medium) in cases {
        let (bus, mut ax, _) = bound_with(Chip { phy, ..healthy() });
        ticks(&mut ax, DOWN_EVERY);
        assert_eq!(
            vendor(&bus).last(),
            Some(&Want::Out(1, 0x22, 2, u16::to_le_bytes(medium).to_vec()))
        );
    }
}

#[test]
fn a_fifo_that_never_drains_costs_restarts_rounds_and_no_more() {
    let (bus, mut ax, _) = bound_with(Chip { phy: 0xA400, fifo: 1 << 30, ..healthy() });
    ticks(&mut ax, DOWN_EVERY);
    let seen = vendor(&bus);
    let rx = seen.iter().filter(|w| matches!(w, Want::Out(1, 0x0b, ..))).count();
    let fifo = seen.iter().filter(|w| matches!(w, Want::In(0x81, ..))).count();
    assert_eq!((rx, fifo, ax.link_up()), (2 * RESTARTS, RESTARTS, true));
}

#[test]
fn a_link_already_set_is_not_written_again() {
    let (bus, mut ax, _) = bound_with(Chip { phy: 0xA400, ..healthy() });
    ticks(&mut ax, DOWN_EVERY);
    bus.0.borrow_mut().calls.clear();
    ticks(&mut ax, LOOK_EVERY * 16 * 2);
    assert!(phy_reads(&bus) >= 1, "the sure look read the PHY");
    assert!(vendor(&bus).iter().all(|w| matches!(w, Want::In(..))), "and wrote nothing");
}
