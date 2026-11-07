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

use nonos_libc::InputEvent;

// Outbound delivery envelope sent to subscribers (separate magic
// from the request channel so subscribers cannot accidentally
// reply on the wrong header). Layout: 4-byte magic NINP + 2-byte
// version + 2-byte flags + 32-byte InputEvent payload.
pub const DELIVERY_MAGIC: u32 = 0x4E49_4E50; // "NINP"
pub const DELIVERY_VERSION: u16 = 1;
const HDR_LEN: usize = 8;
const EVENT_LEN: usize = core::mem::size_of::<InputEvent>();
/// The window id at 40 and a reserved word at 44 follow the event (abi
/// wire.toml [input.delivery_v1]). A receiver that reads 40 bytes reads the
/// frame as it was before; the kernel hands it the first 40.
pub const DELIVERY_LEN: usize = HDR_LEN + EVENT_LEN + 8;
const WINDOW_AT: usize = HDR_LEN + EVENT_LEN;
/// Header flag: the window id names the window the event was routed to.
pub const DELIVERY_FLAG_WINDOW: u16 = 1;
/// Envelope flags: the frame is the router's notice that an unauthenticated
/// Bluetooth device was refused on the secure path (route/notice.rs).
pub const NOTICE_UNPROVEN: u16 = 1;
/// The auto-lock time passed with no input: the shell locks.
pub const NOTICE_IDLE_LOCK: u16 = 2;
/// The event kind a notice carries: none an input device posts.
pub const NOTICE_KIND: u16 = 0xFFFF;

// `window` is the receiver's window a pointer event was routed to, its x and
// y local to it; None for a key, a notice, or an event in screen pixels.
pub fn encode_delivery(out: &mut [u8; DELIVERY_LEN], event: &InputEvent, window: Option<u32>) {
    out[0..4].copy_from_slice(&DELIVERY_MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&DELIVERY_VERSION.to_le_bytes());
    let flags = if window.is_some() { DELIVERY_FLAG_WINDOW } else { 0 };
    out[6..8].copy_from_slice(&flags.to_le_bytes());
    out[8..10].copy_from_slice(&event.kind.to_le_bytes());
    // The kernel's source index stays here (K8b): no app learns which device
    // typed, only the modifiers in the low byte.
    out[10..12].copy_from_slice(&app_flags(event.flags).to_le_bytes());
    out[12..16].copy_from_slice(&event.code.to_le_bytes());
    out[16..20].copy_from_slice(&event.x.to_le_bytes());
    out[20..24].copy_from_slice(&event.y.to_le_bytes());
    out[24..28].copy_from_slice(&event.delta_x.to_le_bytes());
    out[28..32].copy_from_slice(&event.delta_y.to_le_bytes());
    out[32..40].copy_from_slice(&event.timestamp_ns.to_le_bytes());
    out[WINDOW_AT..WINDOW_AT + 4].copy_from_slice(&window.unwrap_or(0).to_le_bytes());
    out[WINDOW_AT + 4..WINDOW_AT + 8].fill(0);
}

/// An event's flags as an app may see them: the modifier byte, without the
/// kernel's source index in the high byte.
pub const fn app_flags(flags: u16) -> u16 {
    flags & 0x00FF
}
