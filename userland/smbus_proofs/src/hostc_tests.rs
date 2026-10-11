// NONOS Operating System (AGPL-3.0-or-later)
//! HOSTC as the driver sets it.

use crate::hostc::hostc_wanted;
use crate::regs::{HOSTC_HST_EN, HOSTC_I2C_EN};

#[test]
fn the_host_is_enabled_and_i2c_timing_turned_off() {
    assert_eq!(hostc_wanted(0), HOSTC_HST_EN);
    assert_eq!(hostc_wanted(HOSTC_I2C_EN), HOSTC_HST_EN);
    assert_eq!(hostc_wanted(HOSTC_HST_EN), HOSTC_HST_EN, "already right: nothing to write");
}

#[test]
fn every_other_bit_is_kept_as_firmware_set_it() {
    // SMB_SMI_EN (bit 1), SSRESET (bit 3) and the byte above.
    assert_eq!(hostc_wanted(0xAB0A), 0xAB0B);
    assert_eq!(hostc_wanted(0xFFFF), 0xFFFB);
}
