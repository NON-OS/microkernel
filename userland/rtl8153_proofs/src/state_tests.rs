// NONOS Operating System (AGPL-3.0-or-later)
//! What the chip holds once bound with its link up, for every code: the
//! drawn address, the filter, one frame per transfer, VLAN tags kept,
//! 1522-byte frames, receive and transmit on, the gate lifted, the power
//! saving off and the PHY advertising every speed.

use nonos_usbnet::mock::drawn;

use crate::chip::{PLA, USB};
use crate::helpers::{up, CODES};

#[test]
fn a_bound_chip_holds_what_the_bring_up_wrote() {
    for code in CODES {
        let (p, _r) = up(code);
        let chip = p.chip.borrow();
        let at = |b, a| chip.stored(b, a);
        let mac = drawn().bytes();
        let b = code >= 0x6000;
        assert_eq!(at(PLA, 0xc000), u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]));
        assert_eq!(at(PLA, 0xc004) & 0xffff, u16::from_le_bytes([mac[4], mac[5]]) as u32);
        assert_eq!(at(PLA, 0xe81c) & 0xff, 0, "{code:04x}: config left unlocked");
        assert_eq!(at(PLA, 0xc010) & 0x0f, 0x0e, "{code:04x}: filter");
        assert_eq!((at(PLA, 0xcd00), at(PLA, 0xcd04)), (u32::MAX, u32::MAX));
        assert_eq!(at(USB, 0xd404) >> 16 & 0x90, 0x10, "{code:04x}: aggregation");
        assert_eq!(at(PLA, 0xe854) & 0x0040, 0, "{code:04x}: VLAN tags stripped");
        assert_eq!(at(PLA, 0xc014) >> 16, 1522);
        assert_eq!(at(PLA, 0xe810) >> 24 & 0x0c, 0x0c, "{code:04x}: receive and transmit");
        assert_eq!(at(PLA, 0xe858) >> 16 & 0x0008, 0, "{code:04x}: receive still gated");
        assert_eq!(at(USB, 0xd808) >> 16 & 0x0001, 0, "{code:04x}: power cut");
        if b {
            assert_eq!(at(USB, 0xcfd8) & 0x0001, 0, "{code:04x}: U1/U2");
            assert_eq!(at(USB, 0xd808) >> 16 & 0x30, 0, "{code:04x}: UPS");
        } else {
            assert_eq!((at(USB, 0xd490), at(USB, 0xd494) & 0xffff), (0, 0), "{code:04x}: U1/U2");
        }
        let phy = |a| chip.phy.get(&a).copied().unwrap_or(0);
        assert_eq!(phy(0xa408) & 0x0de0, 0x0de0, "{code:04x}: advertisement");
        assert_eq!(phy(0xa412) & 0x0300, 0x0200, "{code:04x}: 1000BASE-T");
        assert_eq!(phy(0xa400) & 0x1800, 0x1000, "{code:04x}: BMCR");
        assert_eq!(phy(0xa430) & 0x0004, 0, "{code:04x}: ALDPS");
        assert_eq!(phy(0xa420), 3, "{code:04x}: PHY not on");
    }
}

#[test]
fn the_5c00_to_5c20_chips_get_their_adc_setting_and_the_others_do_not() {
    for code in [0x5c00, 0x5c10, 0x5c20, 0x5c30, 0x6000, 0x6010] {
        let (p, _r) = up(code);
        let adc = p.chip.borrow().phy.get(&0xbc06).copied();
        let want = (code <= 0x5c20).then_some(0x01c0);
        assert_eq!(adc, want, "{code:04x}");
    }
}
