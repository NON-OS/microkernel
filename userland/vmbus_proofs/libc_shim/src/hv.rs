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
use crate::channels::{Interface, Route};
use crate::dma::LIVE;
use crate::vmbus_out::{check, Refusal};

pub const MK_VMBUS_VENDOR: u16 = 0x1414;
pub const MK_VMBUS_DEVICE: u16 = 0x5642;
pub const MK_HV_MESSAGE_BYTES: usize = 256;

const E_AGAIN: i64 = -11;
const E_INVAL: i64 = -22;
const E_PERM: i64 = -1;
const E_NODEV: i64 = -19;
const E_NOENT: i64 = -2;
const E_BUSY: i64 = -16;

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
    /// Records the kernel made for channels: (record, relid, device).
    pub static GIVEN: RefCell<Vec<(u64, u32, u16)>> = const { RefCell::new(Vec::new()) };
}

/// The records the kernel gave channels so far.
pub fn given() -> Vec<(u64, u32, u16)> {
    GIVEN.with(|g| g.borrow().clone())
}

/// A fresh kernel table, for a test that starts again.
pub fn reset_channels() {
    CHANNELS.with(|c| *c.borrow_mut() = Channels::new());
    GIVEN.with(|g| g.borrow_mut().clear());
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
    let held = |gpa: u64| {
        LIVE.with(|l| {
            l.borrow()
                .iter()
                .any(|&(_, d, at, n)| d == device_id && at <= gpa && gpa + 4096 <= at + n)
        })
    };
    match check(connection, ty, payload, held) {
        Ok(()) => {}
        Err(Refusal::Page) => return E_PERM,
        Err(_) => return E_INVAL,
    }
    let rc = h.borrow_mut().post(connection, payload);
    if rc == 0 {
        CHANNELS.with(|c| c.borrow_mut().take_core_message(connection, payload));
    }
    rc
}

/// MkHvChannel as the kernel serves it: a live, unbound channel of an
/// interface a driver may be given, made a record.
pub fn mk_hv_channel(root: u64, relid: u32) -> i64 {
    if !holds(root) {
        return E_PERM;
    }
    CHANNELS.with(|c| {
        let mut c = c.borrow_mut();
        let Some(ch) = c.get(relid).copied() else { return E_NOENT };
        if ch.record.is_some() {
            return E_BUSY;
        }
        let Some(i) = Interface::of(&ch.interface) else { return E_PERM };
        let record = 0x1_0000_1000 + GIVEN.with(|g| g.borrow().len()) as u64;
        if c.bind(relid, record).is_err() {
            return E_NOENT;
        }
        GIVEN.with(|g| g.borrow_mut().push((record, relid, i.device())));
        record as i64
    })
}

pub fn mk_hv_recv(device_id: u64, out: *mut [u8; MK_HV_MESSAGE_BYTES]) -> i64 {
    if !holds(device_id) {
        return E_PERM;
    }
    let Some(h) = host() else { return E_NODEV };
    // A bound channel's answers go to its driver, never to the core.
    let m = loop {
        let Some(m) = h.borrow_mut().recv() else { return E_AGAIN };
        let n = m.len().min(MK_HV_MESSAGE_BYTES);
        if n < 16 || m[..4] != 1u32.to_le_bytes() {
            break m;
        }
        let len = (m[4] as usize).min(n - 16);
        let (route, _) = CHANNELS.with(|c| c.borrow_mut().take_host_message(&m[16..16 + len]));
        if route == Route::Core {
            break m;
        }
    };
    let n = m.len().min(MK_HV_MESSAGE_BYTES);
    // SAFETY: the caller hands a writable 256-byte buffer.
    unsafe { (&mut *out)[..n].copy_from_slice(&m[..n]) };
    n as i64
}
