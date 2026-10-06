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

//! The video the shell asked this player to open (the file manager's Enter
//! on an .avi). The shell keeps the path until the app it launched or focused
//! asks, so the player asks at start and then once a second, which also
//! catches a file sent to a window already open. Once a second, not every
//! tick: a playing video ticks every few milliseconds.

use alloc::string::ToString;

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};
use nonos_libc::mk_uptime_ms;

use super::open_arg_reply::{arg_of, Arg};
use super::state::VideoApp;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_TAKE_OPEN_ARG} and
// image_viewer's poll_open: keep all three identical.
const NDSH: u32 = 0x4E44_5348;
const OP_TAKE_OPEN_ARG: u16 = 0x0008;
const EVERY_MS: i64 = 1000;

impl VideoApp {
    /// Play the file the shell holds for this app, if any. True when the
    /// window changed.
    pub(super) fn poll_open_arg(&mut self) -> bool {
        let now = mk_uptime_ms();
        if now < self.arg_due_ms {
            return false;
        }
        self.arg_due_ms = now + EVERY_MS;
        let Some(shell) = lookup_service(b"desktop_shell") else { return false };
        let mut rx = [0u8; 512];
        let Ok(total) = call_payload(shell.port, NDSH, OP_TAKE_OPEN_ARG, 1, &[], &mut rx) else {
            return false;
        };
        match rx.get(HDR_LEN..total).map_or(Arg::None, arg_of) {
            Arg::None => false,
            Arg::Play(path) => {
                let path = path.to_string();
                self.play_path(path);
                true
            }
            Arg::Refuse => {
                self.status = Some("only Motion-JPEG .avi files play here");
                true
            }
        }
    }
}
