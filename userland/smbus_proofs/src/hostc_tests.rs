// NONOS Operating System (AGPL-3.0-or-later)
//! HOSTC as the driver sets it, and puts it back.

use crate::hostc::{hostc_wanted, turned_on, HostcCfg, HOSTC_CHANGED};
use crate::regs::{HOSTC_HST_EN, HOSTC_I2C_EN};

#[test]
fn the_host_is_enabled_and_i2c_timing_turned_off() {
    assert_eq!(hostc_wanted(0), HOSTC_HST_EN);
    assert_eq!(hostc_wanted(HOSTC_I2C_EN), HOSTC_HST_EN);
    assert_eq!(hostc_wanted(HOSTC_HST_EN), HOSTC_HST_EN, "already right: nothing to write");
}

#[test]
fn every_other_bit_is_kept_as_firmware_set_it() {
    // SMB_SMI_EN (bit 1), SSRESET (bit 3) and the bits above.
    assert_eq!(hostc_wanted(0x0A), 0x0B);
    assert_eq!(hostc_wanted(0xFF), 0xFB);
}

/// HOSTC as the broker serves it: one byte, and a write that takes only the
/// two bits it names.
struct Byte {
    hostc: u8,
    writes: Vec<u8>,
}

impl HostcCfg for Byte {
    fn read(&mut self) -> Option<u8> {
        Some(self.hostc)
    }
    fn write(&mut self, bits: u8) -> bool {
        assert_eq!(bits & !HOSTC_CHANGED, 0, "a bit the broker refuses");
        self.writes.push(bits);
        self.hostc = (self.hostc & !HOSTC_CHANGED) | bits;
        true
    }
}

#[test]
fn a_bring_up_that_fails_after_the_host_is_on_puts_hostc_back() {
    let mut cfg = Byte { hostc: 0x0E, writes: Vec::new() };
    let got: Result<(), _> = turned_on(&mut cfg, || Err("grant refused"));
    assert_eq!(got, Err("grant refused"));
    assert_eq!(cfg.hostc, 0x0E, "I2C mode on and the host off, as firmware left it");
    assert_eq!(cfg.writes, [HOSTC_HST_EN, HOSTC_I2C_EN]);
}

#[test]
fn a_bring_up_that_succeeds_keeps_the_host_on() {
    let mut cfg = Byte { hostc: 0x0A, writes: Vec::new() };
    assert_eq!(turned_on(&mut cfg, || Ok(7)), Ok(7));
    assert_eq!(cfg.hostc, 0x0B);
    let mut right = Byte { hostc: 0x0B, writes: Vec::new() };
    assert_eq!(turned_on(&mut right, || Err::<(), _>("x")), Err("x"));
    assert!(right.writes.is_empty(), "nothing changed, nothing to put back");
}
