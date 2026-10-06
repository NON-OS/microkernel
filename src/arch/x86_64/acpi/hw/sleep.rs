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

//! The register half of entering a sleep state (ACPI 6.5 section 7.4 and
//! 16.1), in the order ACPICA's `hwsleep.c` uses:
//!
//! Legacy (fixed hardware) platforms, `acpi_hw_legacy_sleep`:
//! 1. clear WAK_STS and every other fixed status bit in PM1a/PM1b status;
//! 2. disable every GPE and clear every GPE status bit (no wake GPEs are
//!    armed, so nothing is re-enabled);
//! 3. read PM1 control (PM1a OR PM1b), clear SLP_TYP, SLP_EN and the
//!    write-only GBL_RLS bit, keep everything else (SCI_EN among them);
//! 4. write SLP_TYPa to PM1a control and SLP_TYPb to PM1b control without
//!    SLP_EN (split writes, for hardware that latches SLP_TYP late);
//! 5. flush caches, then write the same values with SLP_EN set, PM1a first.
//!
//! Hardware-reduced platforms (FADT HW_REDUCED_ACPI), `acpi_hw_extended_sleep`:
//! clear WAK_STS in SLEEP_STATUS_REG, then write SLP_TYPa and SLP_EN to
//! SLEEP_CONTROL_REG. PM1 blocks do not exist there.
//!
//! The SLP_TYP values come from the `\_Sx` package (see `aml::sleep_obj`).
//! `_PTS` and `_GTS` are AML methods; running them needs an interpreter this
//! kernel does not have, so they are not run (`_GTS` was removed from the
//! spec in 5.0 and Linux has skipped it by default since 3.4).

use super::fadt_decode::FadtInfo;
use super::gas::{gas_read, gas_write, Gas, RegisterBus};

/// PM1 status bits (ACPI 6.5 table 4.16).
pub const PM1_STS_WAK: u16 = 1 << 15;
/// TMR, BM, GBL, PWRBTN, SLPBTN, RTC and WAK status: ACPICA's
/// `ACPI_BITMASK_ALL_FIXED_STATUS`.
pub const PM1_STS_ALL_FIXED: u16 = 0x8731;

/// PM1 control bits (ACPI 6.5 table 4.18).
pub const PM1_CNT_GBL_RLS: u16 = 1 << 2;
pub const PM1_CNT_SLP_TYP_SHIFT: u16 = 10;
pub const PM1_CNT_SLP_TYP_MASK: u16 = 0x7 << PM1_CNT_SLP_TYP_SHIFT;
pub const PM1_CNT_SLP_EN: u16 = 1 << 13;

/// Sleep control / status register bits (ACPI 6.5 section 4.8.3.7 and 4.8.3.8).
pub const X_SLEEP_TYPE_SHIFT: u8 = 2;
pub const X_SLEEP_TYPE_MASK: u8 = 0x1C;
pub const X_SLEEP_ENABLE: u8 = 0x20;
pub const X_WAKE_STATUS: u8 = 0x80;

/// SLP_TYPa / SLP_TYPb for one sleep state, from the `\_Sx` package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SleepTypes {
    pub a: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepError {
    /// No PM1a control block (legacy) or no SLEEP_CONTROL_REG (reduced).
    NoControlRegister,
    /// A register exists but could not be written.
    WriteFailed,
}

/// Split a PM1 event block into its status (first half) and enable (second
/// half) registers. Each half is `bit_width / 2` wide, as ACPICA's
/// `acpi_tb_setup_fadt_registers` does.
pub fn pm1_halves(evt: &Gas) -> (Gas, Gas) {
    if !evt.is_present() || evt.bit_width < 16 {
        return (Gas::empty(), Gas::empty());
    }
    let half_bits = evt.bit_width / 2;
    let half_bytes = (half_bits / 8) as u64;
    (evt.at_offset(0, half_bits), evt.at_offset(half_bytes, half_bits))
}

/// Disable every GPE in one block and clear its status bits. The block is
/// `len` bytes: the first half status, the second half enable, one byte per
/// register (ACPI 6.5 section 4.8.4.1).
pub fn quiesce_gpe_block<B: RegisterBus>(bus: &mut B, block: &Gas, len: u8) {
    if !block.is_present() || len < 2 {
        return;
    }
    let regs = (len / 2) as u64;
    for i in 0..regs {
        let _ = gas_write(bus, &block.at_offset(regs + i, 8), 0x00);
    }
    for i in 0..regs {
        let _ = gas_write(bus, &block.at_offset(i, 8), 0xFF);
    }
}

/// The PM1 control values to write: (PM1a, PM1b), before SLP_EN is set.
pub fn pm1_control_values(current: u16, types: SleepTypes) -> (u16, u16) {
    let base = current & !(PM1_CNT_SLP_TYP_MASK | PM1_CNT_SLP_EN | PM1_CNT_GBL_RLS);
    let a = base | (((types.a & 0x7) as u16) << PM1_CNT_SLP_TYP_SHIFT);
    let b = base | (((types.b & 0x7) as u16) << PM1_CNT_SLP_TYP_SHIFT);
    (a, b)
}

/// The SLEEP_CONTROL_REG value for a hardware-reduced platform.
pub fn sleep_control_value(types: SleepTypes) -> u8 {
    ((types.a << X_SLEEP_TYPE_SHIFT) & X_SLEEP_TYPE_MASK) | X_SLEEP_ENABLE
}

/// Program the hardware to enter the sleep state `types` describes. On real
/// hardware a successful S5 write does not return; the caller decides what
/// to do if it does. `flush` runs immediately before the SLP_EN write.
pub fn enter_sleep<B: RegisterBus, F: FnMut()>(
    bus: &mut B,
    fadt: &FadtInfo,
    types: SleepTypes,
    mut flush: F,
) -> Result<(), SleepError> {
    if fadt.is_hw_reduced() {
        return enter_sleep_reduced(bus, fadt, types, flush);
    }
    if !fadt.pm1a_cnt.is_accessible() {
        return Err(SleepError::NoControlRegister);
    }

    let (a_sts, _) = pm1_halves(&fadt.pm1a_evt);
    let (b_sts, _) = pm1_halves(&fadt.pm1b_evt);
    let _ = gas_write(bus, &a_sts, PM1_STS_ALL_FIXED as u64);
    if b_sts.is_present() {
        let _ = gas_write(bus, &b_sts, PM1_STS_ALL_FIXED as u64);
    }

    quiesce_gpe_block(bus, &fadt.gpe0, fadt.gpe0_len);
    quiesce_gpe_block(bus, &fadt.gpe1, fadt.gpe1_len);

    let mut current = gas_read(bus, &fadt.pm1a_cnt).unwrap_or(0) as u16;
    if fadt.pm1b_cnt.is_present() {
        current |= gas_read(bus, &fadt.pm1b_cnt).unwrap_or(0) as u16;
    }
    let (a, b) = pm1_control_values(current, types);

    if !gas_write(bus, &fadt.pm1a_cnt, a as u64) {
        return Err(SleepError::WriteFailed);
    }
    if fadt.pm1b_cnt.is_present() {
        let _ = gas_write(bus, &fadt.pm1b_cnt, b as u64);
    }

    flush();

    if !gas_write(bus, &fadt.pm1a_cnt, (a | PM1_CNT_SLP_EN) as u64) {
        return Err(SleepError::WriteFailed);
    }
    if fadt.pm1b_cnt.is_present() {
        let _ = gas_write(bus, &fadt.pm1b_cnt, (b | PM1_CNT_SLP_EN) as u64);
    }
    Ok(())
}

fn enter_sleep_reduced<B: RegisterBus, F: FnMut()>(
    bus: &mut B,
    fadt: &FadtInfo,
    types: SleepTypes,
    mut flush: F,
) -> Result<(), SleepError> {
    if !fadt.sleep_control.is_accessible() {
        return Err(SleepError::NoControlRegister);
    }
    if fadt.sleep_status.is_present() {
        let _ = gas_write(bus, &fadt.sleep_status, X_WAKE_STATUS as u64);
    }
    flush();
    if !gas_write(bus, &fadt.sleep_control, sleep_control_value(types) as u64) {
        return Err(SleepError::WriteFailed);
    }
    Ok(())
}
