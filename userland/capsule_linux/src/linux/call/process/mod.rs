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

/*
 * What a process is and has used: its ids, groups, nice value
 * and execution domain, and its usage.
 */

pub(super) mod ids;
pub(super) mod usage;

pub use ids::{
    getgroups, getpriority, getres, nice_of, personality, setgroups, setpriority, setres,
};
pub use usage::{getrusage, mine as usage_of, sysinfo, times};
