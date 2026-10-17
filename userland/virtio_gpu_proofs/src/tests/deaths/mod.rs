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
//! A dead or silent owner of virtio-gpu's screen, and the driver's own
//! death, at every command (G-73), against the host model.

mod ask;
mod draw;
mod hang;
mod judge;
mod kit;
mod rig;

mod blank_tests;
mod handover_sweep_tests;
pub mod handover_tests;
mod id_tests;
mod orphan_tests;
mod owner_tests;
mod setup_tests;
mod sweep_tests;
mod undo_tests;
