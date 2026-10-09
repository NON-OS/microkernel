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
 * Keys, from the input router. With no window manager there is no focus to
 * route them by, so the installer holds the keyboard the way setup did
 * before it. The router lets only the capsules it names hold it.
 */

use nonos_app_skeleton::clients::input_router::subscribe;
use nonos_app_skeleton::wire::{call_status, NIRS_MAGIC};

const OP_GRAB_REQUEST: u16 = 0x0003;
const KIND_MASK_KEYS: u32 = 0b11;

pub(super) fn subscribe_keys(router: u32, request_id: u32) -> bool {
    subscribe(router, request_id, KIND_MASK_KEYS).is_ok()
}

/* True once the keyboard is this installer's. */
pub(super) fn grab_keyboard(router: u32, request_id: u32) -> bool {
    let mut body = [0u8; 8];
    body[0..4].copy_from_slice(&KIND_MASK_KEYS.to_le_bytes());
    call_status(router, NIRS_MAGIC, OP_GRAB_REQUEST, request_id, &body) == Ok(0)
}
