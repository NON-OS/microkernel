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

//! A window run names its package to the desktop shell before its guest
//! starts, so the dock takes the window it shows for the tile the package
//! is for, or for no app's: a minimised Qwen window is raised by its dock
//! icon (the focus frame reaches control.rs) however Qwen was opened, and a
//! store run's window is not taken for Qwen's. The shell takes the name
//! only from app.linux.run (capsule_desktop_shell
//! server/handlers/linux_window.rs). The call is answered before the guest
//! can open a window, so the window's open notice finds the tile.

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};

use super::run_mode::Mode;

// Hand-synced with capsule_desktop_shell protocol::{MAGIC, OP_LINUX_WINDOW}.
const NDSH: u32 = 0x4E44_5348;
const OP_LINUX_WINDOW: u16 = 0x0013;
/// The longest package name the shell takes (state/taskbar/linux_run.rs).
const PACKAGE_MAX: usize = 64;

/// Tell the shell this run's package, when it is a run in its own window.
/// Nothing the shell answers changes the run: a shell that is not there or
/// refuses leaves the window untracked, as it was before.
pub(super) fn name_to_shell() {
    let Some((name, Mode::Window)) = super::request::run_request() else {
        return;
    };
    let Some(body) = named(name.as_bytes()) else { return };
    let Some(shell) = lookup_service(b"desktop_shell") else {
        return;
    };
    let mut rx = [0u8; HDR_LEN + 4];
    let _ = call_payload(shell.port, NDSH, OP_LINUX_WINDOW, 1, body, &mut rx);
}

/// The body the shell is sent for package `name`: the name itself, or none
/// when it is empty or longer than the shell takes.
fn named(name: &[u8]) -> Option<&[u8]> {
    (!name.is_empty() && name.len() <= PACKAGE_MAX).then_some(name)
}
