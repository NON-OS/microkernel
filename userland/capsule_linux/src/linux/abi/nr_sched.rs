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

//! Linux x86_64 numbers for the scheduler calls and the other epoll forms.
//! Same contract as `nr`, which re-exports them.

pub const SCHED_SETPARAM: u64 = 142;
pub const SCHED_GETPARAM: u64 = 143;
pub const SCHED_SETSCHEDULER: u64 = 144;
pub const SCHED_GETSCHEDULER: u64 = 145;
pub const SCHED_GET_PRIORITY_MAX: u64 = 146;
pub const SCHED_GET_PRIORITY_MIN: u64 = 147;
pub const SCHED_SETAFFINITY: u64 = 203;
pub const EPOLL_CREATE: u64 = 213;
pub const EPOLL_PWAIT2: u64 = 441;
