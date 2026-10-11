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

//! Chip revisions: tg3_asic_rev is the revision ID shifted right by 12,
//! tg3_chip_rev by 8 (tg3.h, near the end).

/// The low revision field that sends tg3_detect_asic_rev to the product-ID
/// register instead.
pub const ASIC_REV_USE_PROD_ID_REG: u32 = 0x0F;
pub const ASIC_REV_57765: u32 = 0x57785;
pub const ASIC_REV_57766: u32 = 0x57766;
pub const CHIPREV_ID_57765_A0: u32 = 0x5778_5000;
/// The value tg3_reset_hw compares tg3_chip_rev against before the
/// transmit-hang fixes. A 57765-class revision ID shifted right by 8 is
/// 0x5778xx or 0x5776xx, never this, so that branch runs on every part of
/// the class in Linux; it is kept that way here.
pub const CHIPREV_57765_AX: u32 = 0x57_7650;

pub fn asic_rev(chip_rev_id: u32) -> u32 {
    chip_rev_id >> 12
}

pub fn chip_rev(chip_rev_id: u32) -> u32 {
    chip_rev_id >> 8
}

/// Whether the revision is one this driver brings up: the 57765 class.
pub fn is_57765_class(chip_rev_id: u32) -> bool {
    matches!(asic_rev(chip_rev_id), ASIC_REV_57765 | ASIC_REV_57766)
}
