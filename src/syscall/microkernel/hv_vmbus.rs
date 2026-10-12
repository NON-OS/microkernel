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

//! The VMBus core's four calls on the SynIC: `MkHvPost`, `MkHvRecv`,
//! `MkHvEvents` and `MkHvSignal`. Each needs Driver and the claim on the
//! VMBus root record; a post is held to the messages the core may send and
//! to pages the caller holds as DMA on that record.

use crate::arch::{
    hv_vmbus_events, hv_vmbus_post, hv_vmbus_recv, hv_vmbus_signal, HvVmbusError, HV_EVENT_WORDS,
    HV_MESSAGE_BYTES,
};
use crate::capabilities::Capability;
use crate::hardware::broker::{dma_covers, vmbus_held_by};
use crate::process::{caps, current_pid};
use crate::syscall::microkernel::errnos::{
    ERRNO_AGAIN, ERRNO_FAULT, ERRNO_INVAL, ERRNO_IO, ERRNO_NODEV, ERRNO_PERM,
};
use crate::usercopy::{copy_from_user, copy_to_user};

/// The largest payload a SynIC message carries.
const PAYLOAD_MAX: usize = HV_MESSAGE_BYTES - 16;

fn holder(device_id: u64) -> Result<u32, i64> {
    let pid = current_pid().ok_or(ERRNO_PERM)?;
    if !caps::has(pid, Capability::Driver.bit()) || !vmbus_held_by(pid, device_id) {
        return Err(ERRNO_PERM);
    }
    Ok(pid)
}

fn errno(e: HvVmbusError) -> i64 {
    match e {
        HvVmbusError::NoSynic => ERRNO_NODEV,
        HvVmbusError::Refused => ERRNO_INVAL,
        HvVmbusError::NotHeld => ERRNO_PERM,
        HvVmbusError::Busy => ERRNO_AGAIN,
        HvVmbusError::Failed => ERRNO_IO,
    }
}

pub fn sys_hv_post(device_id: u64, connection: u64, ty: u64, buf: u64, len: u64) -> i64 {
    let pid = match holder(device_id) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let (Ok(connection), Ok(ty)) = (u32::try_from(connection), u32::try_from(ty)) else {
        return ERRNO_INVAL;
    };
    let len = match usize::try_from(len) {
        Ok(n) if n <= PAYLOAD_MAX => n,
        _ => return ERRNO_INVAL,
    };
    let mut payload = [0u8; PAYLOAD_MAX];
    if copy_from_user(buf, &mut payload[..len]).is_err() {
        return ERRNO_FAULT;
    }
    let held = |gpa: u64| dma_covers(pid, device_id, gpa);
    match hv_vmbus_post(connection, ty, &payload[..len], held) {
        Ok(()) => 0,
        Err(e) => errno(e),
    }
}

/// The oldest message from the host, header and payload, into `out`
/// (256 bytes): its length, or -EAGAIN when none waits.
pub fn sys_hv_recv(device_id: u64, out: u64) -> i64 {
    if let Err(e) = holder(device_id) {
        return e;
    }
    let mut m = [0u8; HV_MESSAGE_BYTES];
    let Some(n) = hv_vmbus_recv(&mut m) else { return ERRNO_AGAIN };
    if copy_to_user(out, &m[..n]).is_err() {
        return ERRNO_FAULT;
    }
    n as i64
}

/// VMBus's channel event flags, up to `words` of them, each handed over
/// and cleared: how many words were written.
pub fn sys_hv_events(device_id: u64, out: u64, words: u64) -> i64 {
    if let Err(e) = holder(device_id) {
        return e;
    }
    let n = (words.min(HV_EVENT_WORDS as u64)) as usize;
    let mut flags = [0u64; HV_EVENT_WORDS];
    let got = hv_vmbus_events(&mut flags[..n]);
    let bytes: [u8; HV_EVENT_WORDS * 8] =
        core::array::from_fn(|i| flags[i / 8].to_le_bytes()[i % 8]);
    if copy_to_user(out, &bytes[..got * 8]).is_err() {
        return ERRNO_FAULT;
    }
    got as i64
}

pub fn sys_hv_signal(device_id: u64, connection: u64) -> i64 {
    if let Err(e) = holder(device_id) {
        return e;
    }
    let Ok(connection) = u32::try_from(connection) else { return ERRNO_INVAL };
    match hv_vmbus_signal(connection) {
        Ok(()) => 0,
        Err(e) => errno(e),
    }
}
