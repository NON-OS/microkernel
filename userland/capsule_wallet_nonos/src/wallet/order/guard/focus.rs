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

/*
 * The wallet's own window and the window manager (T1's wallet side):
 * asked to raise and focus it when the consent opens, through
 * app_skeleton's client, a refusal only waiting (terms/forward.rs); and
 * asked who holds focus with OP_QUERY_FOCUS. Not an answer, or another
 * window's, is not focus.
 */

use nonos_app_skeleton::clients::wm::{window_focus, window_raise};
use nonos_app_skeleton::discover::lookup_port;
use nonos_app_skeleton::wire::{call_payload, read_u32, HDR_LEN, NWMP_MAGIC};
use nonos_libc::{mk_getpid, mk_uptime_ms};

use crate::wallet::manifest::manifest;
use crate::wallet::order::terms::arming::Guard;
use crate::wallet::order::terms::forward::forward;

const WM: &[u8] = b"wm";
/// The window manager's OP_QUERY_FOCUS: its answer, after the status, is
/// the focused window's owner pid and window id.
const OP_QUERY_FOCUS: u16 = 0x000D;
const STATUS_LEN: usize = 4;

fn request_id() -> u32 {
    mk_uptime_ms() as u32
}

/// Ask to raise the wallet's window and give it focus; the consent's guard.
pub fn raise_self() -> Guard {
    let Some(port) = lookup_port(WM) else { return Guard::default() };
    let window = manifest().window_id;
    let raised = window_raise(port, request_id(), window);
    forward(raised, window_focus(port, request_id(), window))
}

/// Whether this wallet's window holds focus, as the window manager says.
pub fn focused() -> bool {
    let Some(port) = lookup_port(WM) else { return false };
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + 8];
    let at = HDR_LEN + STATUS_LEN;
    match call_payload(port, NWMP_MAGIC, OP_QUERY_FOCUS, request_id(), &[], &mut rx) {
        Ok(n) if n >= rx.len() => {
            let (pid, window) = (read_u32(&rx, at), read_u32(&rx, at + 4));
            pid == Ok(mk_getpid()) && window == Ok(manifest().window_id)
        }
        _ => false,
    }
}
