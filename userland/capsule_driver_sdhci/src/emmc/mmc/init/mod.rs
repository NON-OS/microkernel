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

//! From a reset host to a card in transfer state, in JESD84's
//! identification order (as recalled, unverified): power, CMD0, CMD1 until
//! ready, CID, RCA, CSD, select; then EXT_CSD, the user area, speed and
//! width. CMD1 goes first with no window, to learn the card's voltages
//! (power/), and again with the shared one.

mod bring_up;
mod capacity;
pub mod check_ready;
mod op_cond;
mod power;
mod power_on;
mod query_ocr;
mod read_cid;
mod read_ext;
mod select_card;
pub mod step;
mod user_area;
mod voltage;

pub use bring_up::bring_up;
