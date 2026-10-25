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

//! VMBus through the SynIC. Each call needs Driver.
//!
//! - The VMBus core, holding the VMBus root: `MkHvPost`, `MkHvRecv`,
//!   `MkHvEvents` and `MkHvSignal` on the root, and `MkHvChannel` to give a
//!   channel the host offered to a record of its own.
//! - A channel's driver, holding that record: `MkHvEvents` and `MkHvSignal`
//!   on the record, for its channel alone.
//!
//! A post is held to the messages the core may send, and to pages held as
//! DMA: on the root, by the core, for the contact; on a channel's record,
//! by its driver, for that channel's GPADLs. Messages from the host keep
//! the kernel's table of channels, and a withdrawn channel's record leaves.

use crate::arch::{
    hv_vmbus_bind, hv_vmbus_channel_device, hv_vmbus_channel_event, hv_vmbus_channel_of,
    hv_vmbus_events, hv_vmbus_post, hv_vmbus_recv, hv_vmbus_signal, HvHolder, HvRetire,
    HvVmbusError, HV_EVENT_WORDS, HV_MESSAGE_BYTES,
};
use crate::capabilities::Capability;
use crate::hardware::broker::{
    dma_covers, dma_covers_any, register_vmbus_channel, retire_all_vmbus_channels,
    retire_vmbus_channel, settle_vmbus_channel, vmbus_channel_held_by, vmbus_held_by,
};
use crate::process::{caps, current_pid};
use crate::syscall::microkernel::errnos::{
    ERRNO_AGAIN, ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_IO, ERRNO_NODEV, ERRNO_NOENT,
    ERRNO_NOMEM, ERRNO_PERM,
};
use crate::usercopy::{copy_from_user, copy_to_user};

/// The largest payload a SynIC message carries.
const PAYLOAD_MAX: usize = HV_MESSAGE_BYTES - 16;

/// Who called, on what.
enum Caller {
    /// The VMBus core, on the root.
    Core,
    /// A channel's driver, on its record.
    Channel,
}

fn driver() -> Result<u32, i64> {
    let pid = current_pid().ok_or(ERRNO_PERM)?;
    if !caps::has(pid, Capability::Driver.bit()) {
        return Err(ERRNO_PERM);
    }
    Ok(pid)
}

fn core(device_id: u64) -> Result<u32, i64> {
    let pid = driver()?;
    if !vmbus_held_by(pid, device_id) {
        return Err(ERRNO_PERM);
    }
    Ok(pid)
}

fn caller(device_id: u64) -> Result<Caller, i64> {
    let pid = driver()?;
    if vmbus_held_by(pid, device_id) {
        Ok(Caller::Core)
    } else if vmbus_channel_held_by(pid, device_id) {
        Ok(Caller::Channel)
    } else {
        Err(ERRNO_PERM)
    }
}

fn errno(e: HvVmbusError) -> i64 {
    match e {
        HvVmbusError::NoSynic => ERRNO_NODEV,
        HvVmbusError::Refused => ERRNO_INVAL,
        HvVmbusError::NotHeld => ERRNO_PERM,
        HvVmbusError::NoChannel | HvVmbusError::Unknown => ERRNO_NOENT,
        HvVmbusError::Busy => ERRNO_AGAIN,
        HvVmbusError::Failed => ERRNO_IO,
        HvVmbusError::Bound => ERRNO_BUSY,
        HvVmbusError::Interface => ERRNO_PERM,
    }
}

pub fn sys_hv_post(device_id: u64, connection: u64, ty: u64, buf: u64, len: u64) -> i64 {
    let pid = match core(device_id) {
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
    let held = |h: HvHolder, gpa: u64| match h {
        HvHolder::Root => dma_covers(pid, device_id, gpa),
        HvHolder::Record(record) => dma_covers_any(record, gpa),
    };
    match hv_vmbus_post(connection, ty, &payload[..len], held) {
        Ok(()) => 0,
        Err(e) => errno(e),
    }
}

/// The oldest message from the host, header and payload, into `out`
/// (256 bytes): its length, or -EAGAIN when none waits. A channel the
/// message withdrew leaves the broker's table on the way.
pub fn sys_hv_recv(device_id: u64, out: u64) -> i64 {
    if let Err(e) = core(device_id) {
        return e;
    }
    let mut m = [0u8; HV_MESSAGE_BYTES];
    let Some((n, retire)) = hv_vmbus_recv(&mut m) else { return ERRNO_AGAIN };
    match retire {
        HvRetire::None => {}
        HvRetire::Record(r) => retire_vmbus_channel(r),
        HvRetire::All => retire_all_vmbus_channels(),
        HvRetire::Settle(r) => settle_vmbus_channel(r),
    }
    if copy_to_user(out, &m[..n]).is_err() {
        return ERRNO_FAULT;
    }
    n as i64
}

/// On the root: VMBus's event flags but for the channels given to drivers,
/// up to `words` of them, each handed over and cleared. On a channel's
/// record: one word, 1 if its channel's flag was set, which is cleared.
/// How many words were written.
pub fn sys_hv_events(device_id: u64, out: u64, words: u64) -> i64 {
    let mut flags = [0u64; HV_EVENT_WORDS];
    let got = match caller(device_id) {
        Err(e) => return e,
        Ok(_) if words == 0 => 0,
        Ok(Caller::Core) => {
            let n = (words.min(HV_EVENT_WORDS as u64)) as usize;
            hv_vmbus_events(&mut flags[..n])
        }
        Ok(Caller::Channel) => {
            let Some(c) = hv_vmbus_channel_of(device_id) else { return ERRNO_NODEV };
            flags[0] = u64::from(hv_vmbus_channel_event(c.relid));
            1
        }
    };
    let bytes: [u8; HV_EVENT_WORDS * 8] =
        core::array::from_fn(|i| flags[i / 8].to_le_bytes()[i % 8]);
    if copy_to_user(out, &bytes[..got * 8]).is_err() {
        return ERRNO_FAULT;
    }
    got as i64
}

/// On the root: signal the host on `connection`. On a channel's record:
/// signal on the connection the host's offer named, and `connection` must
/// be 0.
pub fn sys_hv_signal(device_id: u64, connection: u64) -> i64 {
    let connection = match caller(device_id) {
        Err(e) => return e,
        Ok(Caller::Core) => match u32::try_from(connection) {
            Ok(c) => c,
            Err(_) => return ERRNO_INVAL,
        },
        Ok(Caller::Channel) if connection != 0 => return ERRNO_INVAL,
        Ok(Caller::Channel) => match hv_vmbus_channel_of(device_id) {
            Some(c) => c.connection,
            None => return ERRNO_NODEV,
        },
    };
    match hv_vmbus_signal(connection) {
        Ok(()) => 0,
        Err(e) => errno(e),
    }
}

/// Give the channel the host offered as `relid` a record of its own, for
/// its driver to claim: the record's id. Only for a relid offered and not
/// withdrawn, not already given, of an interface a driver may be given.
pub fn sys_hv_channel(root: u64, relid: u64) -> i64 {
    if let Err(e) = core(root) {
        return e;
    }
    let Ok(relid) = u32::try_from(relid) else { return ERRNO_INVAL };
    let device = match hv_vmbus_channel_device(relid) {
        Ok(d) => d,
        Err(e) => return errno(e),
    };
    let Some(record) = register_vmbus_channel(device) else { return ERRNO_NOMEM };
    match hv_vmbus_bind(relid, record) {
        Ok(_) => record as i64,
        Err(e) => {
            retire_vmbus_channel(record);
            errno(e)
        }
    }
}
