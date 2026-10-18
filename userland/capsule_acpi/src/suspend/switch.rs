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

//! Decision 6, the owner's: whether NONOS suspends at all. This is the one
//! switch for S3 and suspend-to-idle, and it is off. While it is off,
//! PREPARE_SLEEP answers states 3 and 0 with EINVAL exactly as it always
//! has, and nothing else in `suspend/` runs. Turning it on is not enough
//! to sleep: the kernel's half (the FACS waking vector, PM1 control, the
//! resume path) answers NotWired until it is built.

pub const SUSPEND: bool = false;

/// The devices allowed to wake the system from S3, by namespace path: the
/// owner's policy, empty until it is set. A device not named here is never
/// armed, whatever its _PRW says.
pub const WAKE_DEVICES: &[&[u8]] = &[];
