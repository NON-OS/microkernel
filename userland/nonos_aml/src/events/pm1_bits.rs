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

//! The PM1 fixed event bits.

/// Status bits (table 4.16); the enables (table 4.17) sit at the same bits.
pub const TMR: u16 = 1 << 0;
pub const GBL: u16 = 1 << 5;
pub const PWRBTN: u16 = 1 << 8;
pub const SLPBTN: u16 = 1 << 9;
pub const RTC: u16 = 1 << 10;
/// Status only: the platform woke from a sleep state.
pub const WAK: u16 = 1 << 15;
