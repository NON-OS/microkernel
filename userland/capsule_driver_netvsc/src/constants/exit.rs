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

//! How the driver leaves. A channel that came up and that the host later
//! broke is a device present that failed: the bring-up policy's
//! EXIT_GAVE_UP (libc bringup/policy.rs), never EXIT_ABSENT, which the
//! kernel tells as no device present.

/// The heap could not be set up.
pub const HEAP_INIT: i32 = 1;
/// The host broke the channel: everything was given back.
pub const CHANNEL_BROKEN: i32 = 6;
