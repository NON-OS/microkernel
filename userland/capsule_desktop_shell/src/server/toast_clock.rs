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
//! The one clock the toasts are timed on: uptime, the clock the serve loop's
//! tick and its inbox wait already use. Never the wall clock, which reads as
//! an error before the RTC is read and steps back when NTP corrects it
//! (`state/toast.rs` has the reasoning).

use nonos_libc::mk_uptime_ms;

use crate::state::toast::UptimeMs;

pub fn now() -> UptimeMs {
    UptimeMs(mk_uptime_ms())
}
