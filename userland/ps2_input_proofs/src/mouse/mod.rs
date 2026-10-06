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

//! The mouse packet path, each file the shipping one: the bytes the aux port
//! delivers are assembled into packets, decoded, and queued. Posting to the
//! kernel input ring talks to the kernel and stays out.

#[path = "../../../capsule_driver_ps2_input/src/mouse/axis.rs"]
mod axis;
#[path = "../../../capsule_driver_ps2_input/src/mouse/event.rs"]
pub mod event;
#[path = "../../../capsule_driver_ps2_input/src/mouse/packet.rs"]
pub mod packet;
#[path = "../../../capsule_driver_ps2_input/src/mouse/parser.rs"]
pub mod parser;
#[path = "../../../capsule_driver_ps2_input/src/mouse/ring.rs"]
pub mod ring;
