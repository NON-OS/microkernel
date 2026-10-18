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

//! Byte offsets of the FADT fields the event layer reads (ACPI 6.5, 5.2.9,
//! table 5.9).

pub const SCI_INT: usize = 46;
pub const SMI_CMD: usize = 48;
pub const ACPI_ENABLE: usize = 52;
pub const ACPI_DISABLE: usize = 53;
pub const PM1A_EVT_BLK: usize = 56;
pub const PM1B_EVT_BLK: usize = 60;
pub const PM1A_CNT_BLK: usize = 64;
pub const PM1B_CNT_BLK: usize = 68;
pub const GPE0_BLK: usize = 80;
pub const GPE1_BLK: usize = 84;
pub const PM1_EVT_LEN: usize = 88;
pub const PM1_CNT_LEN: usize = 89;
pub const GPE0_BLK_LEN: usize = 92;
pub const GPE1_BLK_LEN: usize = 93;
pub const GPE1_BASE: usize = 94;
pub const FLAGS: usize = 112;
pub const X_PM1A_EVT_BLK: usize = 148;
pub const X_PM1B_EVT_BLK: usize = 160;
pub const X_PM1A_CNT_BLK: usize = 172;
pub const X_PM1B_CNT_BLK: usize = 184;
pub const X_GPE0_BLK: usize = 220;
pub const X_GPE1_BLK: usize = 232;
/// The first FADT (ACPI 1.0) ends after the flags.
pub const MIN_LEN: usize = 116;
/// Flags bit 20: no fixed hardware, so no SCI blocks (4.1).
pub const HW_REDUCED_ACPI: u32 = 1 << 20;
