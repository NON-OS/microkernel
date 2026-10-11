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

//! The bus voltage. The host says what it can supply (Capabilities 26:24),
//! the card what it accepts (its OCR). As Linux does (mmc_power_up, then
//! mmc_select_voltage), the bus is first powered at the highest voltage the
//! host offers, and once the card's OCR is known, at the lowest voltage both
//! share.

mod ocr;
mod select;
mod vdd;

pub use ocr::{host_ocr, vdd_for_bit, EMMC_DUAL_OCR};
pub use select::{first_vdd, select};
pub use vdd::Vdd;
