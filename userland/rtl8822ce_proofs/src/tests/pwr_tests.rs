// NONOS Operating System (AGPL-3.0-or-later)
//! The 8822C's power tables are rtw88's PCIe entries, in order, and the
//! shared power-on runs them: emulation then enable from cold, the disable
//! first on a card a warm reboot left powered.

#[path = "../../../rtl8821ce_proofs/src/pwr_mock.rs"]
mod pwr_mock;

use crate::pwr::command::{PwrCmd, CMD_END, CMD_POLL, CMD_WRITE};
use crate::pwr::{power_on, power_on_then_master, NoMaster, PowerOn};
use crate::pwr::{CARD_DISABLE, CARD_EMULATE, CARD_ENABLE, CR_UNPOWERED};
use pwr_mock::Chip;

type Row = (char, u16, u8, u8);

fn rows(t: &[PwrCmd]) -> Vec<Row> {
    let kind = |c| match c {
        CMD_WRITE => 'w',
        CMD_POLL => 'p',
        CMD_END => 'e',
        _ => '?',
    };
    t.iter().map(|c| (kind(c.cmd), c.offset, c.mask, c.value)).collect()
}

#[rustfmt::skip]
const EMULATE: [Row; 5] = [('w', 0x2E, 0x04, 0x04), ('w', 0x2D, 0x01, 0), ('w', 0x7F, 0x80, 0),
    ('w', 0x05, 0x98, 0), ('e', 0, 0, 0)];
#[rustfmt::skip]
const ENABLE: [Row; 20] = [('w', 0x05, 0x1C, 0), ('w', 0x75, 1, 1), ('p', 0x06, 2, 2),
    ('w', 0x75, 1, 0), ('w', 0x2E, 8, 0), ('w', 0x06, 1, 1), ('w', 0x05, 0x18, 0),
    ('w', 0x1018, 4, 4), ('w', 0x05, 1, 1), ('p', 0x05, 1, 0), ('w', 0x74, 0x20, 0x20),
    ('w', 0x71, 0x10, 0), ('w', 0x62, 0xE0, 0xE0), ('w', 0x61, 0xE0, 0), ('w', 0x1F, 0xC0, 0x80),
    ('w', 0xEF, 0xC0, 0x80), ('w', 0x1045, 0x10, 0x10), ('w', 0x10, 4, 4), ('w', 0x1064, 2, 2),
    ('e', 0, 0, 0)];
#[rustfmt::skip]
const DISABLE: [Row; 16] = [('w', 0x93, 8, 0), ('w', 0x1F, 0xFF, 0), ('w', 0xEF, 0xFF, 0),
    ('w', 0x1045, 0x10, 0), ('w', 0x49, 2, 0), ('w', 0x06, 1, 1), ('w', 0x02, 2, 0),
    ('w', 0x05, 2, 2), ('p', 0x05, 2, 0), ('w', 0x67, 0x20, 0), ('w', 0x81, 0xC0, 0),
    ('w', 0x90, 2, 0), ('w', 0x92, 0xFF, 0x20), ('w', 0x93, 0xFF, 0x04), ('w', 0x05, 4, 4),
    ('e', 0, 0, 0)];

#[test]
fn the_tables_are_rtw88s_pcie_entries() {
    assert_eq!(rows(CARD_EMULATE), EMULATE);
    assert_eq!(rows(CARD_ENABLE), ENABLE);
    assert_eq!(rows(CARD_DISABLE), DISABLE);
    assert_eq!(CR_UNPOWERED, 0xEA);
}

#[test]
fn a_cold_card_is_emulated_then_enabled() {
    let chip = Chip::new(CR_UNPOWERED);
    assert!(matches!(power_on(&chip), PowerOn::FromCold));
    let regs = chip.regs.borrow();
    assert_eq!(regs[0x1018] & 4, 4);
    assert_eq!(regs[0x1045] & 0x10, 0x10);
    assert_eq!(regs[0x1064] & 2, 2);
    assert_eq!(regs[0xEF] & 0xC0, 0x80);
    assert!(!chip.wrote(0x92), "no disable on a cold card");
}

#[test]
fn a_warm_card_is_disabled_first_and_kept_off_the_bus_if_it_stays_on() {
    let chip = Chip::new(0x00);
    let r = power_on_then_master(&chip, || panic!("bus master on a card left on"));
    assert_eq!(r.err(), Some(NoMaster::LeftOn { cr_after_off: 0x00 }));
    assert_eq!(chip.regs.borrow()[0x92], 0x20);
    assert_eq!(chip.regs.borrow()[0x93] & 0x04, 0x04);
}
