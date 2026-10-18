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

//! The dock's timers (state/taskbar: the launch pulse, the brand reveal,
//! the wait for a launched app's window) take uptime only, the `Uptime`
//! type, and a reading behind the time one was set ends it rather than hold
//! it. They were fed the wall clock: before the RTC was read it read an
//! error, and NTP stepped it back by hours on a laptop whose RTC keeps local
//! time, so a pulse or reveal set before the step stayed for its length.

mod pulse;
mod reveal;
mod rig;
