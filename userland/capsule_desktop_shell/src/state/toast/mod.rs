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

//! One toast: what it says, how loud, and when it goes away.
//!
//! Every time here is on the uptime clock (`mk_uptime_ms`, read through
//! `server::toast_clock`), never the wall clock. The wall clock reads as an
//! error until the RTC has been read, and steps back when NTP corrects it (an
//! RTC kept in local time is hours ahead on many laptops): a toast stamped on
//! it stayed up for as long as the clock stood still, or for the length of
//! the step back.

mod item;
mod times;

pub use item::{Toast, TOAST_TEXT_MAX};
pub use times::{UptimeMs, TOAST_HELD_MS, TOAST_LIFETIME_MS};
