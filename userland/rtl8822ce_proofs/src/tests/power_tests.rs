// NONOS Operating System (AGPL-3.0-or-later)
//! rtw8822c_phy_set_param's prologue: the BB resets released, RF enabled and
//! out of reset, BB/RF on in WLRF1, DPD off for every rate; the header init
//! with both three-wire interfaces on and CCK/OFDM off before the tables and
//! on after; the baseband reset pulsed release, assert, release.

use crate::model::Chip;
use crate::phy::power::{bb_reset, header_init, power_on};
use crate::regs::Mmio;

#[test]
fn power_on_sets_rtw88s_bits() {
    let c = Chip::new();
    c.set32(0x0A70, 0xFFFF_F000);
    power_on(&c);
    assert_eq!(c.read8(0x0002) & 0x03, 0x03);
    assert_eq!(c.read8(0x001F) & 0x07, 0x07);
    assert_eq!(c.read32(0x00EC) & 0x0700_0000, 0x0700_0000);
    assert_eq!(c.read32(0x0A70), 0xFFFF_F3FF, "DIS_DPD_RATEALL, the rest kept");
}

#[test]
fn the_header_init_brackets_the_tables() {
    let c = Chip::new();
    c.set32(0x1C3C, 0xFF);
    header_init(&c, true);
    for wire in [0x180C, 0x410C] {
        assert_eq!(c.read32(wire), 1 << 28 | 0x3, "{wire:#x}");
    }
    assert_eq!(c.read32(0x1C3C), 0xFC, "CCK and OFDM off before");
    header_init(&c, false);
    assert_eq!(c.read32(0x1C3C), 0xFF, "and on after");
    assert_eq!(c.first(0x180C), Some(0), "path A's interface first");
}

#[test]
fn the_bb_reset_is_a_pulse() {
    let c = Chip::new();
    c.mem.borrow_mut().insert(0x0003, 0xD8);
    bb_reset(&c);
    assert_eq!(c.at(0x0002), [0xD801, 0xD800, 0xD801]);
}
