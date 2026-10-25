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

//! Who may change the capture indicator, on or off: only a process holding
//! the capture capability bit for that kind, asked of the kernel per request
//! (MkCapCheck), so nobody can spoof it either way. Neither bit is ambient
//! and no capsule may hold one until the owner decides Decision 9, so until
//! then the indicator never lights and no capture can be announced.

use crate::state::indicator::{KIND_CAMERA, KIND_MICROPHONE};

/// The kernel's CameraCapture bit (the lead's 7e835e4ee; libc
/// CAP_CAMERA_CAPTURE).
pub const CAMERA_CAPTURE: Option<u64> = Some(1 << 37);
/// The kernel's MicrophoneCapture bit (libc CAP_MICROPHONE_CAPTURE).
pub const MICROPHONE_CAPTURE: Option<u64> = Some(1 << 38);

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
