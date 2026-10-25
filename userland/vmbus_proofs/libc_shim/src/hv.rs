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

//! The VMBus core's SynIC calls as the kernel serves them: refused unless
//! the caller holds the VMBus root, and a post held to the kernel's own
//! check and table of channels (both included by path), with the caller's
//! live grants on the root as the pages it holds, and anyone's on a
//! channel's record as that channel's. What passes goes to the host the
//! test installs; what the host sends feeds the same table.

use std::cell::RefCell;
use std::rc::Rc;

use crate::broker::CLAIMED;
use crate::channels::Channels;
use crate::dma::LIVE;
use crate::vmbus_out::{check, Holder, Refusal};

pub const MK_VMBUS_VENDOR: u16 = 0x1414;
pub const MK_VMBUS_DEVICE: u16 = 0x5642;
pub const MK_HV_MESSAGE_BYTES: usize = 256;

const E_AGAIN: i64 = -11;
const E_INVAL: i64 = -22;
const E_PERM: i64 = -1;
const E_NODEV: i64 = -19;
const E_NOENT: i64 = -2;

/// The host on the other side of the SynIC.
pub trait Host {
    /// A message the kernel let through: 0, or -EAGAIN for no room.
    fn post(&mut self, connection: u32, payload: &[u8]) -> i64;
    /// The next message for the guest, header and payload.
    fn recv(&mut self) -> Option<Vec<u8>>;
}

thread_local! {
    static HOST: RefCell<Option<Rc<RefCell<dyn Host>>>> = const { RefCell::new(None) };
    /// The kernel's table of channels, as the kernel keeps it.
    pub(crate) static CHANNELS: RefCell<Channels> = const { RefCell::new(Channels::new()) };
}

pub fn set_host(h: Rc<RefCell<dyn Host>>) {
    HOST.with(|s| *s.borrow_mut() = Some(h));
}

fn host() -> Option<Rc<RefCell<dyn Host>>> {
    HOST.with(|s| s.borrow().clone())
}

fn holds(device_id: u64) -> bool {
    CLAIMED.with(|c| c.get() == Some(device_id))
}

pub fn mk_hv_post(device_id: u64, connection: u32, ty: u32, buf: *const u8, len: usize) -> i64 {
    if !holds(device_id) {
        return E_PERM;
    }
    let Some(h) = host() else { return E_NODEV };
    if len > MK_HV_MESSAGE_BYTES - 16 {
        return E_INVAL;
    }
    // SAFETY: the caller hands `len` readable bytes, as the real call requires.
    let payload = unsafe { std::slice::from_raw_parts(buf, len) };
    let on = |dev: u64, gpa: u64| {
        LIVE.with(|l| {
            l.borrow().iter().any(|&(_, d, at, n)| d == dev && at <= gpa && gpa + 4096 <= at + n)
        })
    };
    let held = |h: Holder, gpa: u64| match h {
        Holder::Root => on(device_id, gpa),
        Holder::Record(r) => on(r, gpa),
    };
    let checked = CHANNELS.with(|c| check(connection, ty, payload, &c.borrow(), held));
    match checked {
        Ok(()) => {}
        Err(Refusal::Page) => return E_PERM,
        Err(Refusal::Channel) => return E_NOENT,
        Err(_) => return E_INVAL,
    }
    let rc = h.borrow_mut().post(connection, payload);
    if rc == 0 {
        CHANNELS.with(|c| c.borrow_mut().from_guest(payload));
    }
    rc
}

pub fn mk_hv_recv(device_id: u64, out: *mut [u8; MK_HV_MESSAGE_BYTES]) -> i64 {
    if !holds(device_id) {
        return E_PERM;
    }
    let Some(h) = host() else { return E_NODEV };
    let Some(m) = h.borrow_mut().recv() else { return E_AGAIN };
    let n = m.len().min(MK_HV_MESSAGE_BYTES);
    if n >= 16 && m[..4] == 1u32.to_le_bytes() {
        let len = (m[4] as usize).min(n - 16);
        CHANNELS.with(|c| c.borrow_mut().from_host(&m[16..16 + len]));
    }
    // SAFETY: the caller hands a writable 256-byte buffer.
    unsafe { (&mut *out)[..n].copy_from_slice(&m[..n]) };
    n as i64
}
