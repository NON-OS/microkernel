// NONOS Operating System (AGPL-3.0-or-later)
//! The bring-up's order on the modelled chip, for every code: settings
//! loaded before the PHY is touched, the PHY powered before LAN-on is
//! waited for, the address written inside its unlock window, every frame
//! refused before the MAC resets, receive gated until the link is up.

use crate::chip::{Access, PLA};
use crate::helpers::{bound, first, last, wrote, CODES};

fn is_phy(a: &Access) -> bool {
    matches!(a, Access::PhyRead(_) | Access::PhyWrite(..))
}

#[test]
fn the_wake_waits_for_the_settings_then_powers_the_phy_then_waits_for_it() {
    for code in CODES {
        let (p, _r) = bound(code);
        let loaded = first(&p, |a| *a == Access::Read(PLA, 0xe004)).unwrap();
        assert!(loaded < first(&p, is_phy).unwrap(), "{code:04x}");
        let power =
            first(&p, |a| matches!(*a, Access::PhyWrite(0xa400, v) if v & 0x0800 == 0)).unwrap();
        let autoneg = first(&p, |a| matches!(*a, Access::PhyWrite(0xa408, _))).unwrap();
        let log = p.chip.borrow().log.clone();
        let lan_on = log[..autoneg].iter().rposition(|a| *a == Access::PhyRead(0xa420)).unwrap();
        assert!(power < lan_on && lan_on < autoneg, "{code:04x}: LAN-on not waited for");
    }
}

#[test]
fn the_address_is_written_unlocked_then_locked_before_the_mac_starts() {
    for code in CODES {
        let (p, _r) = bound(code);
        let low = first(&p, |a| matches!(*a, Access::Write(PLA, 0xc000, 0xf, _))).unwrap();
        let high = first(&p, |a| matches!(*a, Access::Write(PLA, 0xc004, 0x3, _))).unwrap();
        let log = p.chip.borrow().log.clone();
        let window = |a: &Access| matches!(*a, Access::Write(PLA, 0xe81c, 0x1, _));
        let unlock = log[..low].iter().rposition(window).unwrap();
        let lock = high + log[high..].iter().position(window).unwrap();
        assert_eq!(
            (log[unlock], log[lock]),
            (Access::Write(PLA, 0xe81c, 1, 0xc0), Access::Write(PLA, 0xe81c, 1, 0))
        );
        assert!(unlock + 1 == low && low + 1 == high && high + 1 == lock, "{code:04x}");
        let reset = first(&p, wrote(PLA, 0xe810, 0x1000_0000)).unwrap();
        assert!(lock < reset, "{code:04x}");
    }
}

#[test]
fn every_frame_is_refused_before_each_reset_and_the_fifos_set_after() {
    for code in CODES {
        let (p, _r) = bound(code);
        let refuse =
            first(&p, |a| matches!(*a, Access::Write(PLA, 0xc010, _, v) if v & 0x0f == 0)).unwrap();
        let reset = first(&p, wrote(PLA, 0xe810, 0x1000_0000)).unwrap();
        let again = last(&p, wrote(PLA, 0xe810, 0x1000_0000)).unwrap();
        let fifo = first(&p, |a| matches!(*a, Access::Write(PLA, 0xc0a0, _, _))).unwrap();
        assert!(refuse < reset && reset < again && again < fifo, "{code:04x}");
    }
}
