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
//! virtio-gpu on the display wire, against the host model: the
//! takeover, MODES and STATE, flushes, mode sets, the host's display
//! events, and a dead host, a killed driver or a silent owner at every
//! command, each judged from the host's own side.

pub mod ask;
pub mod kit;
pub mod rig;
pub mod wrong;

mod events_tests;
mod flush_tests;
mod limbo_tests;
mod mode_set_tests;
mod mode_sweep_tests;
mod silence_sweep_tests;
mod take_tests;
