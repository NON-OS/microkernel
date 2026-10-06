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

//! The fixed-feature power button (ACPI 6.5 section 4.8.3.1), decided and
//! read without AML.
//!
//! When the FADT's PWR_BUTTON flag is clear the button is a fixed feature:
//! a press latches PWRBTN_STS in the PM1 status register whether or not an
//! interrupt is wired for it, and writing the bit back as one clears it. So
//! a press can be found by reading the register, with no SCI handler and no
//! interpreter. A control-method button (PWR_BUTTON set, a PNP0C0C device)
//! is reported through Notify from AML and cannot be seen this way, and
//! with SCI_EN clear the firmware has not handed ACPI to the OS, so the
//! button still belongs to its SMI handler.

use super::fadt_decode::FadtInfo;
use super::gas::{Gas, SPACE_SYSTEM_IO};
use super::sleep::pm1_halves;

/// FADT flags bit 4: the power button is a control-method device.
pub const FADT_PWR_BUTTON: u32 = 1 << 4;
/// PM1 status bit 8: the power button was pressed.
pub const PM1_STS_PWRBTN: u16 = 1 << 8;
/// PM1 control bit 0: the platform is in ACPI mode.
pub const PM1_CNT_SCI_EN: u16 = 1 << 0;
/// A press within this long of the last is the same press bouncing, or the
/// button held: one press, one shutdown request.
pub const DEBOUNCE_MS: u64 = 1_000;

/// What the FADT says about the power button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    /// Fixed feature, PM1 status at these I/O ports (B is 0 when absent).
    Fixed { sts_a: u16, sts_b: u16 },
    /// A PNP0C0C device, reported through AML.
    ControlMethod,
    /// A hardware-reduced platform: no PM1 blocks; the button is a GPIO or
    /// a generic event device, also through AML.
    HwReduced,
    /// A PM1 status register NONOS reads only from System I/O (every x86
    /// PC), or none at all.
    Unreachable,
}

/// The PM1 status register's port when it can be read with one 16-bit I/O
/// cycle: System I/O, 16 bits wide, at bit 0.
fn io_status_port(evt: &Gas) -> Option<u16> {
    let (sts, _) = pm1_halves(evt);
    let ok = sts.space == SPACE_SYSTEM_IO
        && sts.bit_width == 16
        && sts.bit_offset == 0
        && sts.address != 0
        && sts.address <= u16::MAX as u64;
    ok.then_some(sts.address as u16)
}

pub fn classify(fadt: &FadtInfo) -> Button {
    if fadt.is_hw_reduced() {
        return Button::HwReduced;
    }
    if fadt.flags & FADT_PWR_BUTTON != 0 {
        return Button::ControlMethod;
    }
    match io_status_port(&fadt.pm1a_evt) {
        Some(sts_a) => Button::Fixed { sts_a, sts_b: io_status_port(&fadt.pm1b_evt).unwrap_or(0) },
        None => Button::Unreachable,
    }
}

pub const fn acpi_mode(pm1_cnt: u16) -> bool {
    pm1_cnt & PM1_CNT_SCI_EN != 0
}

pub const fn pressed(pm1_sts: u16) -> bool {
    pm1_sts & PM1_STS_PWRBTN != 0
}

/// What to write to PM1 status to clear the press and nothing else: the
/// register is write-one-to-clear, so every other bit is written as zero.
pub const fn clear_value() -> u16 {
    PM1_STS_PWRBTN
}

/// Whether a press at `now_ms` is a new one, given the last accepted press:
/// one press per `DEBOUNCE_MS`.
pub const fn fresh(last_ms: Option<u64>, now_ms: u64) -> bool {
    match last_ms {
        Some(last) => now_ms.saturating_sub(last) >= DEBOUNCE_MS,
        None => true,
    }
}
