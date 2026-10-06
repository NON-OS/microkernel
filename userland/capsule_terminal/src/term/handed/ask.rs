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

//! Ask the desktop shell whether it holds a command for this window.
//!
//! Only the shell can answer: `desktop_shell` is a name the kernel gives the
//! shell capsule at spawn, and no other capsule may register it
//! (register_allowed.rs). The shell keeps a command only from a Launchpad
//! tool tile and answers it only to a Terminal window; see
//! desktop_shell's take_open_arg.rs.

use alloc::vec::Vec;

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};

use crate::term::dimensions::LINE_MAX;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_TAKE_OPEN_ARG} and the
// editor's open_arg.rs: keep them identical.
const NDSH: u32 = 0x4E44_5348;
const OP_TAKE_OPEN_ARG: u16 = 0x0008;

/// Room for the status, the longest prefix and the longest line.
const RX: usize = HDR_LEN + 4 + 8 + LINE_MAX;

/// The reply after the wire header, or None when the shell is not there.
pub fn ask() -> Option<Vec<u8>> {
    let shell = lookup_service(b"desktop_shell")?;
    let mut rx = [0u8; RX];
    let total = call_payload(shell.port, NDSH, OP_TAKE_OPEN_ARG, 1, &[], &mut rx).ok()?;
    rx.get(HDR_LEN..total).map(<[u8]>::to_vec)
}
