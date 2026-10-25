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

//! The scheduler calls, answered as Linux answers an unprivileged process
//! on the one CPU the guest is shown. Scheduling is the kernel's: a policy a
//! guest names changes nothing, and a real-time one is refused, since no
//! guest holds the privilege Linux asks for it.

mod param;
mod policy;

pub use param::{priority_bound, sched_getparam, sched_setaffinity, sched_setparam};
pub use policy::{sched_getscheduler, sched_setscheduler};
