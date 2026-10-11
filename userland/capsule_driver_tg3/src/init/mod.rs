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

//! Programming the part once setup holds every grant, in tg3_reset_hw's
//! order: firmware told, PHY and chip reset, the 57765 fixups, rings,
//! buffer manager, station address, engines, receiver, PHY, filters.

pub mod abort;
pub mod announce;
pub mod blocks;
pub mod bufmgr;
pub mod coalesce;
pub mod engines;
pub mod filters;
pub mod finish;
pub mod fixups;
pub mod grc;
pub mod mac_setup;
pub mod modes;
pub mod power;
pub mod rcbs;
pub mod reset;
pub mod rings;
pub mod run;
pub mod rx_filter;
pub mod stand_down;
pub mod station_address;
pub mod wait;

pub use finish::finish;
