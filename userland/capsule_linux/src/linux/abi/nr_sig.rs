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

//! Syscall numbers for process lifecycle, signals and timers, transcribed
//! from the x86_64 table.

pub const PAUSE: u64 = 34;
pub const GETITIMER: u64 = 36;
pub const ALARM: u64 = 37;
pub const SETITIMER: u64 = 38;
pub const KILL: u64 = 62;
pub const RT_SIGPENDING: u64 = 127;
pub const RT_SIGTIMEDWAIT: u64 = 128;
pub const RT_SIGQUEUEINFO: u64 = 129;
pub const RT_SIGSUSPEND: u64 = 130;
pub const TKILL: u64 = 200;
pub const TIMER_CREATE: u64 = 222;
pub const TIMER_SETTIME: u64 = 223;
pub const TIMER_GETTIME: u64 = 224;
pub const TIMER_GETOVERRUN: u64 = 225;
pub const TIMER_DELETE: u64 = 226;
pub const TGKILL: u64 = 234;
pub const WAITID: u64 = 247;
pub const RT_TGSIGQUEUEINFO: u64 = 297;
pub const SIGNALFD: u64 = 282;
pub const SIGNALFD4: u64 = 289;
