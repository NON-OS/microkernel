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

//! The folder the shell asked this window to show: a folder icon opened on
//! the desktop. The shell keeps it until the app it launched or focused asks,
//! so the manager asks at start and then on each tick, which also catches a
//! folder sent to a window that was already open.

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};

use super::navigate::navigate;
use super::open_arg_reply::reply_dir;
use super::state::{Mode, State};

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_TAKE_OPEN_ARG} and
// image_viewer's poll_open: keep all three identical.
const NDSH: u32 = 0x4E44_5348;
const OP_TAKE_OPEN_ARG: u16 = 0x0008;

/// Browse the folder the shell holds for this app, if any. True when the
/// window changed.
pub fn poll_open_arg(state: &mut State) -> bool {
    // Not while a prompt, filter or help is up: the shell keeps the folder,
    // and it is taken on the first tick after the user is done.
    if !matches!(state.mode, Mode::Browse | Mode::Preview) {
        return false;
    }
    let Some(shell) = lookup_service(b"desktop_shell") else { return false };
    let mut rx = [0u8; 512];
    let Ok(total) = call_payload(shell.port, NDSH, OP_TAKE_OPEN_ARG, 1, &[], &mut rx) else {
        return false;
    };
    let Some(dir) = rx.get(HDR_LEN..total).and_then(reply_dir) else { return false };
    state.mode = Mode::Browse;
    navigate(state, &dir);
    true
}
