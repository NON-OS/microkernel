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

mod dead;
mod on_cpu;
mod on_cpu_space;
mod on_cpu_switch;
mod on_cpu_wake;
mod select;
mod switching;
mod thread_asid;

pub(crate) use dead::is_dead;
pub use on_cpu::{cpu_holding, cpu_running};
pub use on_cpu_space::cpu_on_tables;
pub(crate) use on_cpu_space::leave_address_space;
pub(crate) use on_cpu_switch::{adopt_current, release_leaving};
pub(crate) use on_cpu_wake::wake_for;
pub use select::{select_next_process, LAST_SCHEDULED_PID};
pub(crate) use switching::switch_to_process;
