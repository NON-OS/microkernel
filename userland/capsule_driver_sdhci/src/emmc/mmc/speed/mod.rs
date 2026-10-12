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

//! Bus timing and width after identification. Timing first: the width
//! test reads EXT_CSD at the clock the card will run at, so a width that
//! passes passes where it will be used. High Speed SDR, if both sides
//! offer it (EXT_CSD DEVICE_TYPE and the host's capabilities), then the
//! widest bus whose EXT_CSD reads back intact. HS200, HS400 and DDR52 are
//! not attempted: the first two need tuning, and SDR at 52 MHz on eight
//! lines already moves about 50 MB/s.

mod bus_width;
mod high_speed;
mod select;

pub use select::select;
