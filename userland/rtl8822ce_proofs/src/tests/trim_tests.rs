// NONOS Operating System (AGPL-3.0-or-later)
//! The factory trims as rtw88 writes them: the thermal byte's bits moved,
//! a path never trimmed ending the thermal and 2.4 GHz PA-bias loops (the
//! 5 GHz bias written unchecked), the TX gain table from the burned trims
//! only, and DPD off either way.

use crate::efuse::facts::Trim;
use crate::model::Chip;
use crate::phy::bits::read32_mask;
use crate::phy::rf::set_paths;
use crate::phy::trim::power::power_gains;
use crate::phy::trim::{power, thermal, thermal_value};
use crate::regs::Mmio;

pub(super) const RF_A: usize = 0x3C00;
const RF_B: usize = 0x4C00;

pub(super) fn trim() -> Trim {
    let tx_5g = [[0xFF; 5]; 2];
    Trim { tx_2g: [0xFF; 3], tx_5g, thermal: [0xFF; 2], pa_bias_2g: [0xFF; 2], pa_bias_5g: [9, 10] }
}

#[test]
fn the_thermal_bits_move_and_an_untrimmed_path_stops() {
    assert_eq!(thermal_value(0b0001), 0b1000);
    assert_eq!(thermal_value(0b1110), 0b0111);
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    thermal(&c, &Trim { thermal: [0b0011, 0xFF], ..trim() }, 2);
    assert_eq!(read32_mask(&c, RF_A + (0x43 << 2), 0xF_0000), 0b1001);
    assert!(c.at(RF_B + (0x43 << 2)).is_empty());
}

#[test]
fn the_gains_are_the_burned_trims() {
    assert_eq!(power_gains(&trim(), 2), None);
    let t = Trim { tx_2g: [0x21, 0xFF, 0x65], ..trim() };
    let g = power_gains(&t, 2).expect("trimmed");
    assert_eq!((g[0][0], g[1][0], g[0][1], g[0][2], g[1][2]), (1, 2, 0, 5, 6));
    let mut t5 = trim();
    t5.tx_5g[1][4] = 0x3F;
    assert_eq!(power_gains(&t5, 2).map(|g| g[1][7]), Some(0x1F), "five bits");
    assert_eq!(power_gains(&t5, 1), None, "path B's 5 GHz trims of a one-path chip");
}

#[test]
fn the_gain_table_is_written_in_rtw88s_order_and_dpd_is_off() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    power(&c, &Trim { tx_2g: [0x21, 0x43, 0x65], ..trim() }, 2);
    let gains = c.at(RF_A + (0x3F << 2));
    assert_eq!(gains, [1, 3, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "entries 0 to 0xE");
    assert_eq!(c.at(RF_A + (0x33 << 2)), (0..15).collect::<Vec<u32>>());
    assert_eq!(c.read32(0x0A70) & 0x3FF, 0x3FF);
    let c = Chip::new();
    power(&c, &trim(), 2);
    assert!(c.at(RF_A + (0x3F << 2)).is_empty() && c.read32(0x0A70) & 0x3FF == 0x3FF);
}
