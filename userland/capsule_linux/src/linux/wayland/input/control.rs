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

//! The desktop shell's control frame. Every app's process is sent a focus-self
//! frame when the person chooses the app from the dock (desktop_shell
//! launcher_request/frame.rs, app_skeleton runner/control.rs). A native app
//! answers it by restoring, raising and focusing its window; this capsule does
//! the same for the guest's shown window (present/hide.rs), which is the only
//! thing that brings a minimised guest window back. Only the desktop shell's
//! own pid is believed, as the window manager's notice believes only its own
//! (focus.rs).

use nonos_app_skeleton::discover::lookup_service;

use crate::linux::guest::Guest;

/// NCTL control frames, version 1 (app_skeleton runner/control.rs).
const CONTROL_LEN: usize = 8;
const CONTROL_MAGIC: u32 = u32::from_le_bytes(*b"NCTL");
const CONTROL_VERSION: u16 = 1;
const OP_FOCUS_SELF: u16 = 1;

/// A frame the inbox did not route as input: the desktop shell's focus-self,
/// if it is one from the shell. True when it was taken, so the pump does not
/// also offer it to the window manager notice (focus.rs).
pub fn took(guest: &mut Guest, frame: &[u8], sender: u32) -> bool {
    if frame.len() != CONTROL_LEN {
        return false;
    }
    let magic = u32::from_le_bytes([frame[0], frame[1], frame[2], frame[3]]);
    let version = u16::from_le_bytes([frame[4], frame[5]]);
    let op = u16::from_le_bytes([frame[6], frame[7]]);
    if magic != CONTROL_MAGIC || version != CONTROL_VERSION || op != OP_FOCUS_SELF {
        return false;
    }
    if !from_desktop_shell(sender) {
        return false;
    }
    super::super::present::hide::restore(guest);
    true
}

/// True only when `sender` is the pid that owns the `desktop_shell` service.
fn from_desktop_shell(sender: u32) -> bool {
    lookup_service(b"desktop_shell").map(|peer| peer.pid) == Some(sender)
}
