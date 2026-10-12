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

//! Who may light the capture indicator: only a process holding the capture
//! capability bit for that kind, asked of the kernel per request. The bits
//! are the lead's to add to the capability table (CameraCapture when B
//! reaches UVC); until a bit is named here every sender is refused, so no
//! capture can be announced, and Decision 9's default keeps START refused.

use crate::state::indicator::{KIND_CAMERA, KIND_MICROPHONE};

/// The kernel's CameraCapture bit, once the table has it.
pub const CAMERA_CAPTURE: Option<u64> = None;
/// The kernel's microphone capture bit, once the table has it.
pub const MICROPHONE_CAPTURE: Option<u64> = None;

/// The capability mask `kind` needs, None when it has none yet.
pub fn needs(kind: u32) -> Option<u64> {
    match kind {
        KIND_CAMERA => CAMERA_CAPTURE,
        KIND_MICROPHONE => MICROPHONE_CAPTURE,
        _ => None,
    }
}

/// Whether `sender` may change `kind`, given how the kernel answers a
/// capability check on it.
pub fn may_indicate(sender: u32, kind: u32, holds: impl Fn(u32, u64) -> bool) -> bool {
    sender != 0 && needs(kind).is_some_and(|mask| holds(sender, mask))
}
