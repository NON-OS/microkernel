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

mod end_note;
pub mod end_rule;
mod exit_and_yield;
mod finalize;
mod park;
mod pending;
pub mod postmortem;
mod purge;
mod reap_log;
mod stop_elsewhere;
mod teardown;

pub use exit_and_yield::exit_and_yield;
pub(crate) use park::park as park_dead;
pub(crate) use pending::drain as drain_pending_teardowns;
pub(crate) use purge::purge_for_new_pid;
pub(crate) use reap_log::{peek_exit_status, reap_exit_status, reap_exit_status_for};
pub use teardown::teardown;
