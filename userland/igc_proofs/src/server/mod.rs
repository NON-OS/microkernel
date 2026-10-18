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

//! The server: the loop, the handlers, the one reply path, and the transmit
//! watchdog, against the IPC calls the shim scripts.

#[path = "../../../capsule_driver_igc/src/server/error.rs"]
pub mod error;
#[path = "../../../capsule_driver_igc/src/server/handlers/mod.rs"]
pub mod handlers;
#[path = "../../../capsule_driver_igc/src/server/runner.rs"]
pub mod runner;
#[path = "../../../capsule_driver_igc/src/server/tx_watch.rs"]
pub mod tx_watch;
