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

//! The AMD FCH GPIO bank's per-pin dword, one per pin at the window's
//! start plus four times the pin. Every bit below is recalled from AMD's
//! published BIOS and Kernel Developer's Guides and Processor Programming
//! References (the GPIO bank section), which could not be read here:
//! unverified.

/// 1 level-triggered, 0 edge-triggered.
pub const LEVEL_TRIG: u32 = 1 << 8;
/// Two bits: 0 active high, 1 active low, 2 both edges.
pub const ACTIVE_LEVEL_SHIFT: u32 = 9;
pub const ACTIVE_LEVEL_MASK: u32 = 3 << ACTIVE_LEVEL_SHIFT;
pub const INTERRUPT_ENABLE: u32 = 1 << 11;
/// Set: the pin's interrupt is delivered; clear: held back.
pub const INTERRUPT_DELIVER: u32 = 1 << 12;
pub const PIN_STS: u32 = 1 << 16;
pub const OUTPUT_VALUE: u32 = 1 << 22;
pub const OUTPUT_ENABLE: u32 = 1 << 23;
/// Write one to clear.
pub const INTERRUPT_STS: u32 = 1 << 28;
/// Write one to clear; never cleared here by accident.
pub const WAKE_STS: u32 = 1 << 29;

/// The bank's master register, in the dword a 64th pin would use, and its
/// end-of-interrupt bit, written after the pending pins are cleared so the
/// bank raises its line again.
pub const MASTER: u32 = 0xFC;
pub const MASTER_EOI: u32 = 1 << 29;
