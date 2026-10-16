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

//! A controller taken, brought up and let go, through the capsule's own
//! setup and teardown against the shim's broker: bus mastering held off
//! until the engines are stopped, every way an attempt ends, the order the
//! teardown stops the controller in before the release, and the discovery
//! and console lines around them.

pub mod attempt;
mod attempt_reach_tests;
mod attempt_tests;
pub mod board;
mod claim_refused_tests;
mod claim_tests;
mod discover_tests;
pub mod given_back;
mod mark_tests;
mod quiesce_order_tests;
mod refusal_master_tests;
mod reset_hold_tests;
mod stop_tests;
pub mod teardown;
mod teardown_fault_tests;
pub mod teardown_log;
mod teardown_tests;
pub mod witness;
