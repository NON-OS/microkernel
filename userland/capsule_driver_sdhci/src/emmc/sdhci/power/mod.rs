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
//! the card what it accepts (its OCR). Power first goes on at the highest
//! voltage the host has, which every card class must at least answer its
//! OCR query at; once the OCR is known, the bus moves to the lowest voltage
//! both share, which draws the least, and the card is told exactly the OCR
//! bits that name that supply.

mod ocr;
mod select;
mod vdd;

pub use ocr::{host_ocr, vdd_for_bit, EMMC_DUAL_OCR, OCR_32_34};
pub use select::{first_vdd, select};
pub use vdd::Vdd;
