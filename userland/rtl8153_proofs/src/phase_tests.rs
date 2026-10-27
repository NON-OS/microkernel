// NONOS Operating System (AGPL-3.0-or-later)
//! The phases of a bind in their order, and a link-up that fails midway
//! leaving the link down to be tried again.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Nic;

use crate::chip::{Access, PLA};
use crate::helpers::{bound, first, last, link_to, wrote, CODES, GIGA_UP};

#[test]
fn version_then_configuration_then_wake_station_autoneg_and_start() {
    for code in CODES {
        let (p, _r) = bound(code);
        let calls = p.bus.0.borrow().calls.clone();
        let version = calls.iter().position(|c| matches!(c, Call::In(s, 4) if s.value == 0xe610));
        let config = calls.iter().position(|c| matches!(c, Call::Out(s, _) if s.request == 0x09));
        assert!(version.unwrap() < config.unwrap(), "{code:04x}: configured before identified");
        let autoload = first(&p, |a| *a == Access::Read(PLA, 0xe004)).unwrap();
        let station = first(&p, |a| matches!(*a, Access::Write(PLA, 0xc000, _, _))).unwrap();
        let phy_reset = first(&p, |a| matches!(*a, Access::PhyWrite(0xa400, v) if v & 0x8000 != 0));
        let mac_reset = first(&p, wrote(PLA, 0xe810, 0x1000_0000)).unwrap();
        // The dword holds CONFIG6 too, which the wake edits: the link look is the last.
        let link = last(&p, |a| *a == Access::Read(PLA, 0xe908)).unwrap();
        let order = [autoload, station, phy_reset.unwrap(), mac_reset, link];
        assert!(order.windows(2).all(|w| w[0] < w[1]), "{code:04x}: {order:?}");
    }
}

#[test]
fn a_link_up_that_fails_midway_reads_down_and_is_tried_again() {
    let (p, mut r) = bound(0x5c20);
    let n = p.chip.borrow().transfers;
    p.chip.borrow_mut().fail_after = Some(n + 3);
    assert_eq!(link_to(&p, &mut r, GIGA_UP), Err(-71));
    assert!(!r.link_up(), "up before receive was on");
    p.chip.borrow_mut().fail_after = None;
    link_to(&p, &mut r, GIGA_UP).unwrap();
    assert!(r.link_up());
}
