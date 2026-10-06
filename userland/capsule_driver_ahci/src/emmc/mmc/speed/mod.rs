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

//! Bus timing and width after identification, as Linux's mmc_select_timing
//! takes them for a host without HS200 tuning: High Speed SDR first
//! (mmc_select_hs, then mmc_set_bus_speed), then the widest bus whose
//! EXT_CSD reads back the same (mmc_select_bus_width with
//! mmc_compare_ext_csds). HS200, HS400 and DDR52 are not attempted: the
//! first two need tuning, and SDR at 52 MHz on eight lines already moves
//! about 50 MB/s.

mod bus_width;
mod high_speed;
mod select;

pub use select::select;
