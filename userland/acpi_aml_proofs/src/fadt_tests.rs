// NONOS Operating System (AGPL-3.0-or-later)
//! FADT decoding: length bounded, X_ fields preferred, checksum advisory.

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::hw::fadt_decode::*;
use crate::arch::x86_64::acpi::hw::gas::*;

pub struct FadtBuilder(pub Vec<u8>);

impl FadtBuilder {
    pub fn new(len: usize) -> Self {
        let mut t = alloc::vec![0u8; len];
        t[0..4].copy_from_slice(b"FACP");
        t[4..8].copy_from_slice(&(len as u32).to_le_bytes());
        t[8] = if len >= 244 { 6 } else { 1 };
        Self(t)
    }
    pub fn u8(mut self, at: usize, v: u8) -> Self {
        self.0[at] = v;
        self
    }
    pub fn u16(mut self, at: usize, v: u16) -> Self {
        self.0[at..at + 2].copy_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u32(mut self, at: usize, v: u32) -> Self {
        self.0[at..at + 4].copy_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u64(mut self, at: usize, v: u64) -> Self {
        self.0[at..at + 8].copy_from_slice(&v.to_le_bytes());
        self
    }
    pub fn gas(mut self, at: usize, g: Gas) -> Self {
        self.0[at] = g.space;
        self.0[at + 1] = g.bit_width;
        self.0[at + 2] = g.bit_offset;
        self.0[at + 3] = g.access_size;
        self.0[at + 4..at + 12].copy_from_slice(&g.address.to_le_bytes());
        self
    }
    pub fn sealed(mut self) -> Vec<u8> {
        self.0[9] = 0;
        let sum = self.0.iter().fold(0u8, |a, &b| a.wrapping_add(b));
        self.0[9] = 0u8.wrapping_sub(sum);
        self.0
    }
}

/// A Gemini Lake (Celeron N4120, HP 15s-fq0xxx class) FADT, revision 6:
/// PM1a at 0x400/0x404, PM timer 0x408, GPE0 0x420 (32 bytes), reset
/// register 0xCF9 = 0x06, RESET_REG_SUP set, X_ blocks filled in.
pub fn gemini_lake_fadt() -> Vec<u8> {
    FadtBuilder::new(276)
        .u32(off::DSDT, 0x7AB3_6000)
        .u64(off::X_DSDT, 0x7AB3_6000)
        .u8(off::PM_PROFILE, 2)
        .u16(off::SCI_INT, 9)
        .u32(off::SMI_CMD, 0xB2)
        .u8(off::ACPI_ENABLE, 0xA0)
        .u8(off::ACPI_DISABLE, 0xA1)
        .u32(off::PM1A_EVT_BLK, 0x400)
        .u32(off::PM1A_CNT_BLK, 0x404)
        .u32(off::PM_TMR_BLK, 0x408)
        .u32(off::GPE0_BLK, 0x420)
        .u8(off::PM1_EVT_LEN, 4)
        .u8(off::PM1_CNT_LEN, 2)
        .u8(off::PM_TMR_LEN, 4)
        .u8(off::GPE0_BLK_LEN, 0x20)
        .u16(off::IAPC_BOOT_ARCH, 0x0003)
        .u32(off::FLAGS, 0x0003_04A5 | FLAG_RESET_REG_SUP)
        .gas(off::RESET_REG, Gas::io(0xCF9, 1))
        .u8(off::RESET_VALUE, 0x06)
        .gas(
            off::X_PM1A_EVT_BLK,
            Gas { space: 1, bit_width: 32, bit_offset: 0, access_size: 2, address: 0x400 },
        )
        .gas(
            off::X_PM1A_CNT_BLK,
            Gas { space: 1, bit_width: 16, bit_offset: 0, access_size: 2, address: 0x404 },
        )
        .gas(
            off::X_PM_TMR_BLK,
            Gas { space: 1, bit_width: 32, bit_offset: 0, access_size: 3, address: 0x408 },
        )
        .gas(
            off::X_GPE0_BLK,
            Gas { space: 1, bit_width: 0, bit_offset: 0, access_size: 1, address: 0x420 },
        )
        .sealed()
}

#[test]
fn a_gemini_lake_fadt_decodes_to_its_registers() {
    let f = decode_fadt(&gemini_lake_fadt()).expect("FACP");
    assert!(f.checksum_ok);
    assert_eq!(f.dsdt, 0x7AB3_6000);
    assert_eq!(f.sci_int, 9);
    assert_eq!(f.pm1a_cnt.address, 0x404);
    assert_eq!(f.pm1a_cnt.bit_width, 16);
    assert!(!f.pm1b_cnt.is_present());
    assert_eq!(f.pm_tmr.address, 0x408);
    assert_eq!(f.gpe0.address, 0x420);
    assert_eq!(f.gpe0_len, 0x20);
    assert!(f.has_reset_reg());
    assert_eq!(f.reset_reg.address, 0xCF9);
    assert_eq!(f.reset_value, 6);
    assert!(!f.is_hw_reduced());
    assert!(f.has_8042());
}

#[test]
fn a_short_acpi1_fadt_never_reads_past_its_length() {
    // 116 bytes declared; the slice carries garbage after it that looks like
    // a reset register and X_ blocks.
    let mut t = FadtBuilder::new(116)
        .u32(off::PM1A_CNT_BLK, 0xB004)
        .u8(off::PM1_CNT_LEN, 2)
        .u32(off::FLAGS, FLAG_RESET_REG_SUP)
        .sealed();
    t.resize(276, 0xEE);
    let f = decode_fadt(&t).unwrap();
    assert!(f.checksum_ok, "checksum covers only the declared length");
    assert_eq!(f.pm1a_cnt, Gas::io(0xB004, 2));
    assert!(!f.reset_reg.is_present());
    assert!(!f.has_reset_reg());
    assert_eq!(f.sleep_control, Gas::empty());
    assert_eq!(f.dsdt, 0);
}

#[test]
fn x_blocks_win_over_legacy_ports_when_nonzero() {
    let t = FadtBuilder::new(244)
        .u32(off::DSDT, 0x1000)
        .u64(off::X_DSDT, 0x1_2345_6000)
        .u32(off::FIRMWARE_CTRL, 0x2000)
        .u64(off::X_FIRMWARE_CTRL, 0x1_0000_0000)
        .u32(off::PM1A_CNT_BLK, 0x604)
        .u8(off::PM1_CNT_LEN, 2)
        .gas(
            off::X_PM1A_CNT_BLK,
            Gas { space: 0, bit_width: 16, bit_offset: 0, access_size: 2, address: 0xFED8_0804 },
        )
        .sealed();
    let f = decode_fadt(&t).unwrap();
    assert_eq!(f.dsdt, 0x1_2345_6000);
    assert_eq!(f.firmware_ctrl, 0x1_0000_0000);
    assert_eq!(f.pm1a_cnt.space, SPACE_SYSTEM_MEMORY);
    assert_eq!(f.pm1a_cnt.address, 0xFED8_0804);
}

#[test]
fn legacy_ports_are_used_when_x_blocks_are_zero() {
    let t = FadtBuilder::new(244)
        .u32(off::DSDT, 0x1000)
        .u32(off::PM1A_EVT_BLK, 0x600)
        .u32(off::PM1B_CNT_BLK, 0x2004)
        .u32(off::PM1A_CNT_BLK, 0x604)
        .u8(off::PM1_EVT_LEN, 4)
        .u8(off::PM1_CNT_LEN, 2)
        .sealed();
    let f = decode_fadt(&t).unwrap();
    assert_eq!(f.dsdt, 0x1000);
    assert_eq!(f.pm1a_evt, Gas::io(0x600, 4));
    assert_eq!(f.pm1a_cnt, Gas::io(0x604, 2));
    assert_eq!(f.pm1b_cnt, Gas::io(0x2004, 2));
}

#[test]
fn wrong_pm1_widths_are_pinned_to_the_fixed_sizes() {
    let t = FadtBuilder::new(244)
        .gas(
            off::X_PM1A_CNT_BLK,
            Gas { space: 1, bit_width: 32, bit_offset: 0, access_size: 3, address: 0x1804 },
        )
        .gas(
            off::X_PM1A_EVT_BLK,
            Gas { space: 1, bit_width: 8, bit_offset: 0, access_size: 1, address: 0x1800 },
        )
        .sealed();
    let f = decode_fadt(&t).unwrap();
    assert_eq!(f.pm1a_cnt.bit_width, 16);
    assert_eq!(f.pm1a_evt.bit_width, 32);
}

#[test]
fn a_bad_checksum_is_reported_not_fatal() {
    let mut t = gemini_lake_fadt();
    t[9] = t[9].wrapping_add(1);
    let f = decode_fadt(&t).unwrap();
    assert!(!f.checksum_ok);
    assert_eq!(f.pm1a_cnt.address, 0x404);
}

#[test]
fn hardware_reduced_fadts_carry_the_sleep_registers() {
    let t = FadtBuilder::new(276)
        .u32(off::FLAGS, FLAG_HW_REDUCED_ACPI)
        .gas(off::SLEEP_CONTROL_REG, Gas::io(0x1008, 1))
        .gas(off::SLEEP_STATUS_REG, Gas::io(0x100C, 1))
        .sealed();
    let f = decode_fadt(&t).unwrap();
    assert!(f.is_hw_reduced());
    assert_eq!(f.sleep_control.address, 0x1008);
}

#[test]
fn something_that_is_not_a_fadt_is_refused() {
    assert!(decode_fadt(b"APIC\x2c\0\0\0").is_none());
    assert!(decode_fadt(&[0u8; 10]).is_none());
    let mut t = FadtBuilder::new(116).sealed();
    t[4..8].copy_from_slice(&8u32.to_le_bytes());
    assert!(decode_fadt(&t).is_none(), "a length below the header is not a table");
}
