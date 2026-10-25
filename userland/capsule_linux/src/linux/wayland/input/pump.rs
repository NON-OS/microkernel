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

//! NONOS input events, as Wayland sees them.
//!
//! Routed, not drained: the personality subscribes to the input router like
//! any app and gets only what arrives while its surface has focus. It holds no
//! InputSource, and a frame from anyone but the router is not believed.

use nonos_app_skeleton::clients::input_router::subscribe;
use nonos_app_skeleton::discover::{from_router, lookup_port};
use nonos_app_skeleton::input::InputEvent;
use nonos_libc::mk_ipc_recv_from;

use crate::linux::guest::Guest;

/// Key down and up, absolute pointer, wheel, button down and up: one bit
/// per input kind (app_skeleton input/decode.rs), wheel being 4.
const KINDS: u32 = 0x7B;
const OWN_INBOX: u64 = 0;
const NOWAIT: u64 = 1;
const NINP_MAGIC: u32 = 0x4E49_4E50;
const HEADER: usize = 8;
/// Frames taken in one pass. A deeper backlog is drained on the next.
const BATCH: usize = 32;

pub fn pump(guest: &mut Guest) {
    if guest.scene.pointer.is_none() && guest.scene.keyboard.is_none() {
        return;
    }
    if !guest.scene.subscribed {
        guest.scene.subscribed =
            lookup_port(b"input_router").is_some_and(|port| subscribe(port, 1, KINDS).is_ok());
    }
    let mut rx = [0u8; HEADER + 32];
    for _ in 0..BATCH {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(OWN_INBOX, rx.as_mut_ptr(), rx.len(), NOWAIT, &mut sender);
        if n < rx.len() as i64 {
            return;
        }
        if u32::from_le_bytes([rx[0], rx[1], rx[2], rx[3]]) != NINP_MAGIC || !from_router(sender) {
            continue;
        }
        if let Some(event) = InputEvent::from_delivery(&rx[HEADER..]) {
            super::deliver::deliver(guest, &event);
        }
    }
}
