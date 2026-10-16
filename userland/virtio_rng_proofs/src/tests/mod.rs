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

//! The driver against a register window, a ring and a buffer in host
//! memory, and a device that answers late or out of turn; and the orders
//! in its source that only a boot runs.

mod bus_master;
mod drive;
mod model;
mod ring;

mod fill_tests;
mod hostile_tests;
mod late_tests;
mod legacy_reset_tests;
mod master_legacy_tests;
mod master_modern_tests;
mod order_tests;
mod queue_tests;
mod reset_tests;
mod ring_tests;
mod status_tests;
mod take_tests;
mod teardown_tests;
