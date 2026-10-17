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

//! Finding the network on the air before a join. The hunt listens: beacons
//! name every network that broadcasts its name, so nothing is transmitted and
//! no saved network's name leaves the machine. Only for a network the person
//! marked hidden (its beacons carry no name) does the hunt send a probe
//! request naming it, once per channel, from an address drawn for this hunt
//! alone (`nonos_wifi_core::address::one_off`); the probe response that
//! answers it carries the name and is taken like a beacon.

mod dwell;
mod find;

pub(super) use find::find_candidates;
