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

//! The adapter's own registers swept: every DISPI value the driver
//! reads is the device's word. A register file is drawn at random
//! (`file`), sometimes with an adapter that does not take the mode written,
//! and the start-up mode is held to what the rules say (`oracle`); the ID
//! check is tried at every value and the EDID window with any bytes.

mod file;
mod lie;
mod oracle;

mod edid_window_tests;
mod id_tests;
mod start_sweep_tests;
