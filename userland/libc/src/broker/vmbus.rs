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

//! The VMBus core's calls on the Hyper-V SynIC. Each takes the VMBus root's
//! device id, which the caller must hold claimed.

use crate::syscall::{call_raw, N_MK_HV_EVENTS, N_MK_HV_POST, N_MK_HV_RECV, N_MK_HV_SIGNAL};

/// The VMBus root's record: Microsoft's vendor id and "VB".
pub const MK_VMBUS_VENDOR: u16 = 0x1414;
pub const MK_VMBUS_DEVICE: u16 = 0x5642;
/// A SynIC message, header and payload.
pub const MK_HV_MESSAGE_BYTES: usize = 256;
/// The event flag words VMBus's SINT has.
pub const MK_HV_EVENT_WORDS: usize = 32;

/// Post a channel message to the host: 0, -EAGAIN when the host has no room
/// now, -EPERM for a page the caller does not hold, -EINVAL for a message
/// the core may not send.
#[no_mangle]
pub extern "C" fn mk_hv_post(
    device_id: u64,
    connection: u32,
    ty: u32,
    buf: *const u8,
    len: usize,
) -> i64 {
    call_raw(N_MK_HV_POST, [device_id, connection as u64, ty as u64, buf as u64, len as u64, 0])
}

/// The oldest message from the host into `out`: its length, or -EAGAIN.
#[no_mangle]
pub extern "C" fn mk_hv_recv(device_id: u64, out: *mut [u8; MK_HV_MESSAGE_BYTES]) -> i64 {
    call_raw(N_MK_HV_RECV, [device_id, out as u64, 0, 0, 0, 0])
}

/// VMBus's event flags, up to `words`, each handed over and cleared: how
/// many words were written.
#[no_mangle]
pub extern "C" fn mk_hv_events(device_id: u64, out: *mut u64, words: usize) -> i64 {
    call_raw(N_MK_HV_EVENTS, [device_id, out as u64, words as u64, 0, 0, 0])
}

/// Signal the host on a channel's connection.
#[no_mangle]
pub extern "C" fn mk_hv_signal(device_id: u64, connection: u32) -> i64 {
    call_raw(N_MK_HV_SIGNAL, [device_id, connection as u64, 0, 0, 0, 0])
}
