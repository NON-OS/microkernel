// NONOS Operating System (AGPL-3.0-or-later)
//! The status endpoint's reports: only a whole one is believed, it only
//! says when to read the PHY, and a PHY that will not answer counts toward
//! the device being given up.

use nonos_usbnet::Nic;

use crate::ax::status::{link_hint, STATUS_LEN};
use crate::ax::watch::{armed, DOWN_EVERY, LOOK_EVERY, SURE_EVERY};
use crate::chip::{healthy, Chip, CONFIG, DEVICE};
use crate::harness::{bound_with, device_of};
use crate::link_tests::ticks;
use crate::seen::phy_reads;

#[test]
fn a_report_is_eight_bytes_and_bit_16_is_the_link() {
    assert_eq!(link_hint(&[0, 0, 1, 0, 0, 0, 0, 0]), Some(true));
    assert_eq!(link_hint(&[0xff, 0xff, 0xfe, 0xff, 0, 0, 0, 0]), Some(false));
    for n in 0..STATUS_LEN {
        assert_eq!(link_hint(&[0xff; 8][..n]), None, "{n} bytes");
    }
    assert_eq!((armed(0), armed(4), armed(8), armed(1024)), (1, 4, 8, STATUS_LEN));
}

#[test]
fn while_up_the_phy_is_read_on_a_down_report_or_a_sure_look_only() {
    let (bus, mut ax, chip) = bound_with(Chip { phy: 0xA400, ..healthy() });
    ticks(&mut ax, DOWN_EVERY);
    bus.0.borrow_mut().calls.clear();
    // Up reports and short ones change nothing; short ones are counted.
    for n in [vec![0, 0, 1, 0, 0, 0, 0, 0], vec![0xff; 4]] {
        bus.0.borrow_mut().notices.push_back(n);
        ticks(&mut ax, LOOK_EVERY);
    }
    assert_eq!((phy_reads(&bus), ax.counts.notices, ax.link_up()), (0, 1, true));
    chip.borrow_mut().phy = 0;
    bus.0.borrow_mut().notices.push_back(vec![0; 8]);
    ticks(&mut ax, LOOK_EVERY);
    assert!(phy_reads(&bus) >= 1 && !ax.link_up());
}

#[test]
fn without_a_status_endpoint_the_phy_is_read_on_every_look() {
    let mut config = CONFIG.to_vec();
    config.drain(18..31); // the interrupt IN and its companion
    config[2] = config.len() as u8;
    config[13] = 2;
    let (bus, found, _) = device_of(DEVICE, vec![config], Chip { phy: 0xA400, ..healthy() });
    let nonos_usbnet::Bind::Ours(mut ax) =
        crate::ax::bind(bus.clone(), &found, nonos_usbnet::mock::drawn())
    else {
        panic!("not bound")
    };
    ticks(&mut ax, DOWN_EVERY);
    bus.0.borrow_mut().calls.clear();
    ticks(&mut ax, LOOK_EVERY * SURE_EVERY);
    assert_eq!(phy_reads(&bus), SURE_EVERY as usize);
}

#[test]
fn a_phy_that_will_not_answer_fails_the_look() {
    let (_, mut ax, chip) = bound_with(healthy());
    chip.borrow_mut().phy_fails = true;
    let r = ticks(&mut ax, DOWN_EVERY * 3);
    assert_eq!(r.iter().filter(|r| r.is_err()).count(), 3);
}
