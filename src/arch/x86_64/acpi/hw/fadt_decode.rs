// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! FADT decoding by byte offset, bounded by the table's own length.
//!
//! The FADT grew with every ACPI revision: 116 bytes in 1.0, 244 in 2.0,
//! 268 in 5.0, 276 in 6.0. Reading the full struct out of a short table reads
//! whatever firmware put after it, which is how a 1.0 FADT ends up with a
//! "reset register" made of the next table's bytes. ACPICA copies the table
//! into a zeroed local of full size (`acpi_tb_create_local_fadt`); this does
//! the same thing field by field: a field that does not fit inside `length`
//! reads as zero.
//!
//! The 64-bit X_ fields win over the 32-bit ones whenever they are nonzero,
//! as in ACPICA's `acpi_tb_select_address` and `acpi_tb_convert_fadt` with
//! the default `acpi_gbl_use32_bit_fadt_addresses = FALSE`. When an X_ block
//! is absent the legacy port and its length byte make a System I/O GAS. The
//! PM1 event, PM1 control and PM timer widths are forced to their fixed sizes
//! (32, 16 and 32 bits) as ACPICA does with `use_default_register_widths`,
//! because firmware gets those bit widths wrong more often than right.

use super::gas::Gas;

/// Byte offsets of the FADT fields (ACPI 6.5 table 5.9).
pub mod off {
    pub const FIRMWARE_CTRL: usize = 36;
    pub const DSDT: usize = 40;
    pub const PM_PROFILE: usize = 45;
    pub const SCI_INT: usize = 46;
    pub const SMI_CMD: usize = 48;
    pub const ACPI_ENABLE: usize = 52;
    pub const ACPI_DISABLE: usize = 53;
    pub const PM1A_EVT_BLK: usize = 56;
    pub const PM1B_EVT_BLK: usize = 60;
    pub const PM1A_CNT_BLK: usize = 64;
    pub const PM1B_CNT_BLK: usize = 68;
    pub const PM_TMR_BLK: usize = 76;
    pub const GPE0_BLK: usize = 80;
    pub const GPE1_BLK: usize = 84;
    pub const PM1_EVT_LEN: usize = 88;
    pub const PM1_CNT_LEN: usize = 89;
    pub const PM_TMR_LEN: usize = 91;
    pub const GPE0_BLK_LEN: usize = 92;
    pub const GPE1_BLK_LEN: usize = 93;
    pub const GPE1_BASE: usize = 94;
    pub const IAPC_BOOT_ARCH: usize = 109;
    pub const FLAGS: usize = 112;
    pub const RESET_REG: usize = 116;
    pub const RESET_VALUE: usize = 128;
    pub const X_FIRMWARE_CTRL: usize = 132;
    pub const X_DSDT: usize = 140;
    pub const X_PM1A_EVT_BLK: usize = 148;
    pub const X_PM1B_EVT_BLK: usize = 160;
    pub const X_PM1A_CNT_BLK: usize = 172;
    pub const X_PM1B_CNT_BLK: usize = 184;
    pub const X_PM_TMR_BLK: usize = 208;
    pub const X_GPE0_BLK: usize = 220;
    pub const X_GPE1_BLK: usize = 232;
    pub const SLEEP_CONTROL_REG: usize = 244;
    pub const SLEEP_STATUS_REG: usize = 256;
}

/// FADT flag bits used by the power code.
pub const FLAG_RESET_REG_SUP: u32 = 1 << 10;
pub const FLAG_HW_REDUCED_ACPI: u32 = 1 << 20;

/// IA-PC boot architecture flag: an 8042 is present.
pub const BOOT_ARCH_8042: u16 = 1 << 1;

/// Smallest table that still carries the ACPI 1.0 fields through FLAGS.
pub const FADT_V1_LEN: u32 = 116;

/// Everything the kernel takes from the FADT, decoded and with the 32/64-bit
/// choice already made.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FadtInfo {
    pub length: u32,
    pub revision: u8,
    /// True when the bytes summed to zero. A bad checksum is reported, not
    /// fatal: Linux warns and carries on, and firmware that patches the FADT
    /// at boot without fixing the sum is common.
    pub checksum_ok: bool,
    pub firmware_ctrl: u64,
    pub dsdt: u64,
    pub pm_profile: u8,
    pub sci_int: u16,
    pub smi_cmd: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub pm1a_evt: Gas,
    pub pm1b_evt: Gas,
    pub pm1a_cnt: Gas,
    pub pm1b_cnt: Gas,
    pub pm_tmr: Gas,
    pub gpe0: Gas,
    pub gpe1: Gas,
    /// GPE block lengths in bytes (status half plus enable half).
    pub gpe0_len: u8,
    pub gpe1_len: u8,
    pub gpe1_base: u8,
    pub boot_arch: u16,
    pub flags: u32,
    pub reset_reg: Gas,
    pub reset_value: u8,
    pub sleep_control: Gas,
    pub sleep_status: Gas,
}

impl FadtInfo {
    pub fn is_hw_reduced(&self) -> bool {
        self.flags & FLAG_HW_REDUCED_ACPI != 0
    }

    /// RESET_REG_SUP set and a reset register actually given.
    pub fn has_reset_reg(&self) -> bool {
        self.flags & FLAG_RESET_REG_SUP != 0 && self.reset_reg.is_present()
    }

    pub fn has_8042(&self) -> bool {
        self.boot_arch & BOOT_ARCH_8042 != 0
    }
}

fn bytes<const N: usize>(t: &[u8], len: usize, at: usize) -> Option<[u8; N]> {
    let end = at.checked_add(N)?;
    if end > len || end > t.len() {
        return None;
    }
    let mut out = [0u8; N];
    out.copy_from_slice(&t[at..end]);
    Some(out)
}

fn u8_at(t: &[u8], len: usize, at: usize) -> u8 {
    bytes::<1>(t, len, at).map(|b| b[0]).unwrap_or(0)
}

fn u16_at(t: &[u8], len: usize, at: usize) -> u16 {
    bytes::<2>(t, len, at).map(u16::from_le_bytes).unwrap_or(0)
}

fn u32_at(t: &[u8], len: usize, at: usize) -> u32 {
    bytes::<4>(t, len, at).map(u32::from_le_bytes).unwrap_or(0)
}

fn u64_at(t: &[u8], len: usize, at: usize) -> u64 {
    bytes::<8>(t, len, at).map(u64::from_le_bytes).unwrap_or(0)
}

fn gas_at(t: &[u8], len: usize, at: usize) -> Gas {
    bytes::<12>(t, len, at).map(|b| Gas::from_bytes(&b)).unwrap_or(Gas::empty())
}

/// `acpi_tb_select_address`: the 64-bit field when nonzero, else the 32-bit.
pub fn select_address(addr32: u32, addr64: u64) -> u64 {
    if addr64 != 0 {
        addr64
    } else {
        addr32 as u64
    }
}

/// Pick the X_ GAS when it carries an address, else build a port GAS from the
/// legacy block, then pin the bit width to `default_bits` when nonzero.
fn select_block(x: Gas, legacy: u32, legacy_len: u8, default_bits: u8) -> Gas {
    let mut g = if x.is_present() {
        x
    } else if legacy != 0 {
        Gas::io(legacy as u64, legacy_len)
    } else {
        Gas::empty()
    };
    if g.is_present() && default_bits != 0 && g.bit_width != default_bits {
        g.bit_width = default_bits;
    }
    g
}

/// Decode a FADT from its bytes. `t` must start at the table header and
/// hold at least the declared length (a shorter slice is treated as the
/// table's end). None only for something that is not a FADT at all.
pub fn decode_fadt(t: &[u8]) -> Option<FadtInfo> {
    if t.len() < 36 || &t[0..4] != b"FACP" {
        return None;
    }
    let declared = u32::from_le_bytes([t[4], t[5], t[6], t[7]]);
    let len = (declared as usize).min(t.len());
    if len < 36 {
        return None;
    }
    let checksum_ok = t[..len].iter().fold(0u8, |a, &b| a.wrapping_add(b)) == 0;

    let pm1_evt_len = u8_at(t, len, off::PM1_EVT_LEN);
    let pm1_cnt_len = u8_at(t, len, off::PM1_CNT_LEN);
    let pm_tmr_len = u8_at(t, len, off::PM_TMR_LEN);
    let gpe0_blk_len = u8_at(t, len, off::GPE0_BLK_LEN);
    let gpe1_blk_len = u8_at(t, len, off::GPE1_BLK_LEN);

    let x_gpe0 = gas_at(t, len, off::X_GPE0_BLK);
    let x_gpe1 = gas_at(t, len, off::X_GPE1_BLK);
    let gpe0 = select_block(x_gpe0, u32_at(t, len, off::GPE0_BLK), gpe0_blk_len, 0);
    let gpe1 = select_block(x_gpe1, u32_at(t, len, off::GPE1_BLK), gpe1_blk_len, 0);
    // The GPE length byte is authoritative (Linux sizes the GPE registers
    // from it); the X_ bit width is the fallback only when the byte is 0.
    let gpe0_len = if gpe0_blk_len != 0 { gpe0_blk_len } else { gpe0.bit_width / 8 };
    let gpe1_len = if gpe1_blk_len != 0 { gpe1_blk_len } else { gpe1.bit_width / 8 };

    Some(FadtInfo {
        length: declared,
        revision: t[8],
        checksum_ok,
        firmware_ctrl: select_address(
            u32_at(t, len, off::FIRMWARE_CTRL),
            u64_at(t, len, off::X_FIRMWARE_CTRL),
        ),
        dsdt: select_address(u32_at(t, len, off::DSDT), u64_at(t, len, off::X_DSDT)),
        pm_profile: u8_at(t, len, off::PM_PROFILE),
        sci_int: u16_at(t, len, off::SCI_INT),
        smi_cmd: u32_at(t, len, off::SMI_CMD),
        acpi_enable: u8_at(t, len, off::ACPI_ENABLE),
        acpi_disable: u8_at(t, len, off::ACPI_DISABLE),
        pm1a_evt: select_block(
            gas_at(t, len, off::X_PM1A_EVT_BLK),
            u32_at(t, len, off::PM1A_EVT_BLK),
            pm1_evt_len,
            32,
        ),
        pm1b_evt: select_block(
            gas_at(t, len, off::X_PM1B_EVT_BLK),
            u32_at(t, len, off::PM1B_EVT_BLK),
            pm1_evt_len,
            32,
        ),
        pm1a_cnt: select_block(
            gas_at(t, len, off::X_PM1A_CNT_BLK),
            u32_at(t, len, off::PM1A_CNT_BLK),
            pm1_cnt_len,
            16,
        ),
        pm1b_cnt: select_block(
            gas_at(t, len, off::X_PM1B_CNT_BLK),
            u32_at(t, len, off::PM1B_CNT_BLK),
            pm1_cnt_len,
            16,
        ),
        pm_tmr: select_block(
            gas_at(t, len, off::X_PM_TMR_BLK),
            u32_at(t, len, off::PM_TMR_BLK),
            pm_tmr_len,
            32,
        ),
        gpe0,
        gpe1,
        gpe0_len,
        gpe1_len,
        gpe1_base: u8_at(t, len, off::GPE1_BASE),
        boot_arch: u16_at(t, len, off::IAPC_BOOT_ARCH),
        flags: u32_at(t, len, off::FLAGS),
        reset_reg: gas_at(t, len, off::RESET_REG),
        reset_value: u8_at(t, len, off::RESET_VALUE),
        sleep_control: gas_at(t, len, off::SLEEP_CONTROL_REG),
        sleep_status: gas_at(t, len, off::SLEEP_STATUS_REG),
    })
}
