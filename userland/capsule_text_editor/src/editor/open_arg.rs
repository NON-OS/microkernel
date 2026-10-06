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

//! The file the shell asked this editor to open: a desktop icon, or the file
//! manager's Enter on a text file. The shell keeps the path until the app it
//! launched or focused asks for it, so the editor asks at start and then on
//! each tick, which also catches a file sent to a window already open.

use alloc::string::String;

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};

use super::app::Editor;
use super::open_arg_reply::reply_path;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_TAKE_OPEN_ARG} and
// image_viewer's poll_open: keep all three identical.
const NDSH: u32 = 0x4E44_5348;
const OP_TAKE_OPEN_ARG: u16 = 0x0008;

fn take_open_arg() -> Option<String> {
    let shell = lookup_service(b"desktop_shell")?;
    let mut rx = [0u8; 512];
    let total = call_payload(shell.port, NDSH, OP_TAKE_OPEN_ARG, 1, &[], &mut rx).ok()?;
    reply_path(rx.get(HDR_LEN..total)?).map(String::from)
}

impl Editor {
    /// Open the file the shell holds for this app, if any. True when the window
    /// changed: the file opened, or the status says why it did not.
    pub(super) fn poll_open_arg(&mut self) -> bool {
        match take_open_arg() {
            Some(path) => {
                let _ = self.open_path(&path);
                true
            }
            None => false,
        }
    }
}
