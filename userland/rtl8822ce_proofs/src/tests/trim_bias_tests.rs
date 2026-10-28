// NONOS Operating System (AGPL-3.0-or-later)
//! The PA bias, written from the 2.4 GHz trims of both paths or not at all
//! (rtw88 checks only those), and the crystal doubler restarted when busy.

use super::trim_tests::{trim, RF_A};
use crate::efuse::facts::Trim;
use crate::model::Chip;
use crate::phy::bits::read32_mask;
use crate::phy::rf::set_paths;
use crate::phy::trim::{pa_bias, x2_check};

const RF_B: usize = 0x4C00;

#[test]
fn an_untrimmed_2g_pa_bias_skips_both_bands() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    pa_bias(&c, &trim(), 2);
    assert!(c.writes.borrow().is_empty());
    pa_bias(&c, &Trim { pa_bias_2g: [0x17, 0x18], ..trim() }, 2);
    assert_eq!(read32_mask(&c, RF_A + (0x60 << 2), 0xF_F000), 0x97);
    assert_eq!(read32_mask(&c, RF_B + (0x60 << 2), 0xF_F000), 0xA8);
}

#[test]
fn a_busy_crystal_doubler_is_restarted() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    x2_check(&c);
    assert!(c.writes.borrow().is_empty());
    c.set32(RF_A + (0xB8 << 2), 1 << 15);
    x2_check(&c);
    assert_eq!(c.at(RF_A + (0xB8 << 2)), [0xC4440, 0x80440]);
    assert_eq!(c.at(RF_A + (0xBA << 2)), [0x6840D]);
}
