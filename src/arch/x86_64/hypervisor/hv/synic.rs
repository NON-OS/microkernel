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
 * The SynIC on VP 0, set up once: the guest OS identity, the hypercall
 * MSR, the message and event flag pages, VMBus's SINT at its vector
 * (masked until the vector has a gate), and the SynIC enabled. Then the
 * four things the VMBus core does through the kernel: take a message,
 * take the channel event flags, post a message, signal the host.
 */

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{fence, AtomicU64, Ordering};

use spin::{Mutex, Once};

use super::channels::{BindError, Channel, Channels, Interface, Retire};
use super::features::{Features, LEAF_FEATURES};
use super::hypercall::{call, choose_instruction};
use super::hypercall_abi::{
    control, post_input, signal_input, status, CALL_POST_MESSAGE, CALL_SIGNAL_EVENT,
};
use super::message::{read, Slot, HEADER_BYTES, MESSAGE_BYTES};
use super::pages::{Page, PAGE_BYTES};
use super::regs::{
    page, scontrol_enabled, sint, GUEST_OS_ID, MSR_EOM, MSR_GUEST_OS_ID, MSR_HYPERCALL,
    MSR_SCONTROL, MSR_SIEFP, MSR_SIMP, MSR_SINT0, MSR_VP_INDEX, SINT_VMBUS, VECTOR_HV_SYNIC,
};
use super::ring::Ring;
use super::vmbus_out::{check, Holder, Refusal, MESSAGE_TYPE_CHANNEL};
use crate::arch::x86_64::cpu::{cpuid, rdmsr, wrmsr};

/// The event flags one SINT has: 2048 bits.
pub const EVENT_WORDS: usize = MESSAGE_BYTES / 8;

struct Synic {
    simp: Page,
    siefp: Page,
    post: Mutex<Page>,
}

static SYNIC: Once<Synic> = Once::new();
static RING: Mutex<Ring> = Mutex::new(Ring::new());
static CHANNELS: Mutex<Channels> = Mutex::new(Channels::new());

/// Set the SynIC up on this CPU, which must be VP 0. Once per boot.
pub fn init() -> Result<(), &'static str> {
    if SYNIC.get().is_some() {
        return Ok(());
    }
    let (eax, ebx, _, _) = cpuid(LEAF_FEATURES);
    let offered = Features { eax, ebx };
    if !offered.synic_usable() {
        return Err("no SynIC on offer");
    }
    if rdmsr(MSR_VP_INDEX) != 0 {
        return Err("not on VP 0");
    }
    choose_instruction();
    wrmsr(MSR_GUEST_OS_ID, GUEST_OS_ID);
    let hypercall = Page::new().ok_or("no frame for the hypercall page")?;
    wrmsr(MSR_HYPERCALL, page(hypercall.phys));
    if rdmsr(MSR_HYPERCALL) & 1 == 0 {
        return Err("hypercall page refused");
    }
    let simp = Page::new().ok_or("no frame for the message page")?;
    let siefp = Page::new().ok_or("no frame for the event flags page")?;
    let post = Page::new().ok_or("no frame for the post page")?;
    wrmsr(MSR_SIMP, page(simp.phys));
    wrmsr(MSR_SIEFP, page(siefp.phys));
    wrmsr(MSR_SINT0 + SINT_VMBUS, sint(VECTOR_HV_SYNIC, true));
    wrmsr(MSR_SCONTROL, scontrol_enabled());
    SYNIC.call_once(|| Synic { simp, siefp, post: Mutex::new(post) });
    Ok(())
}

pub fn ready() -> bool {
    SYNIC.get().is_some()
}

/// Let VMBus's SINT raise its vector: for the gate's installer, once the
/// gate exists. Before, the core polls.
pub fn unmask() {
    if SYNIC.get().is_some() {
        wrmsr(MSR_SINT0 + SINT_VMBUS, sint(VECTOR_HV_SYNIC, false));
    }
}

/*
 * Move what the host left in VMBus's slot into the ring, freeing the slot
 * and, when the host has more queued, ending the message so it sends the
 * next. Only on VP 0, whose slot it is and whose EOM counts; the gate's
 * handler runs there, and so does the core's poll on a one-CPU guest.
 */
pub fn service() {
    let Some(s) = SYNIC.get() else { return };
    if rdmsr(MSR_VP_INDEX) != 0 {
        return;
    }
    let slot = s.simp.at(SINT_VMBUS as usize * MESSAGE_BYTES) as *mut [u8; MESSAGE_BYTES];
    let mut ring = RING.lock();
    loop {
        // SAFETY: VMBus's slot in the message page, which the hypervisor
        // writes and only this function, under the ring's lock, clears.
        let m = unsafe { read_volatile(slot) };
        let Slot::Full { pending, .. } = read(&m) else { break };
        if !ring.push(&m) {
            break;
        }
        // SAFETY: as above; the type word frees the slot.
        unsafe { write_volatile(slot as *mut u32, 0) };
        fence(Ordering::SeqCst);
        if !pending {
            break;
        }
        wrmsr(MSR_EOM, 0);
    }
}

/// The oldest message taken, header and payload, its length, and the
/// channel record it retired, if any. A channel message is read into the
/// kernel's own table of channels on its way to the core.
pub fn recv(out: &mut [u8; MESSAGE_BYTES]) -> Option<(usize, Retire)> {
    service();
    let m = RING.lock().pop()?;
    let Slot::Full { ty, len, .. } = read(&m) else { return None };
    let retire = if ty == MESSAGE_TYPE_CHANNEL {
        CHANNELS.lock().from_host(&m[HEADER_BYTES..HEADER_BYTES + len])
    } else {
        Retire::None
    };
    *out = m;
    Some((HEADER_BYTES + len, retire))
}

/// Why a post did not go.
pub enum PostError {
    Refused(Refusal),
    /// The hypercall's status.
    Status(u16),
    NoSynic,
}

/// Check a post against what the core may send and the channels it may
/// build or open, send it, and, once the host took it, note the GPADL it
/// tied to a channel. One lock across all three, so the table the check
/// read is the table the post changes.
pub fn post_checked(
    connection: u32,
    ty: u32,
    payload: &[u8],
    held: impl Fn(Holder, u64) -> bool,
) -> Result<(), PostError> {
    let mut channels = CHANNELS.lock();
    check(connection, ty, payload, &channels, held).map_err(PostError::Refused)?;
    match post(connection, ty, payload) {
        None => Err(PostError::NoSynic),
        Some(super::hypercall_abi::STATUS_SUCCESS) => {
            channels.from_guest(payload);
            Ok(())
        }
        Some(status) => Err(PostError::Status(status)),
    }
}

/// The interface a live, unbound, offered channel would be given as, or
/// why it cannot be.
pub fn channel_interface(relid: u32) -> Result<Interface, BindError> {
    let channels = CHANNELS.lock();
    let c = channels.get(relid).ok_or(BindError::Unknown)?;
    if c.record.is_some() {
        return Err(BindError::Bound);
    }
    Interface::of(&c.interface).ok_or(BindError::Interface)
}

/// The type of the note the kernel leaves the core in its ring: a channel's
/// driver let go while the host may still hold GPADLs of it, and the core
/// is to close the channel and tear them down. The payload is the relid.
/// The host's VMBus messages are type 1; this is no message of the host's.
pub const NOTE_RELEASED: u32 = 0x4E4F_0001;

/// A channel record's grants are being drained: whether their pages must be
/// kept, because the host may still hold a GPADL over them. The first time,
/// the core is left a note to close the channel; if its ring is full, the
/// next drain tries again.
pub fn released(record: u64) -> bool {
    let (busy, first) = CHANNELS.lock().released(record);
    if let Some(relid) = first {
        let mut m = [0u8; MESSAGE_BYTES];
        m[..4].copy_from_slice(&NOTE_RELEASED.to_le_bytes());
        m[4] = 4;
        m[HEADER_BYTES..HEADER_BYTES + 4].copy_from_slice(&relid.to_le_bytes());
        if !RING.lock().push(&m) {
            CHANNELS.lock().unnote(relid);
        }
    }
    busy
}

/// Give a channel to its record.
pub fn bind(relid: u32, record: u64) -> Result<Channel, BindError> {
    CHANNELS.lock().bind(relid, record)
}

/// The channel a record was given.
pub fn channel_of(record: u64) -> Option<Channel> {
    CHANNELS.lock().by_record(record).copied()
}

fn flags_word(i: usize) -> Option<&'static AtomicU64> {
    let s = SYNIC.get()?;
    if i >= EVENT_WORDS {
        return None;
    }
    let base = s.siefp.at(SINT_VMBUS as usize * MESSAGE_BYTES) as *const AtomicU64;
    // SAFETY: word i of VMBus's 256 bytes in the event flags page, a kernel
    // frame for the life of the kernel, which the hypervisor sets bits in
    // atomically and the kernel clears atomically.
    Some(unsafe { &*base.add(i) })
}

/// VMBus's event flags for the core, each word handed over and cleared,
/// but for the bits of channels given to drivers, which stay theirs.
pub fn events(out: &mut [u64]) -> usize {
    if SYNIC.get().is_none() {
        return 0;
    }
    let channels = CHANNELS.lock();
    let n = out.len().min(EVENT_WORDS);
    for (i, w) in out.iter_mut().take(n).enumerate() {
        let keep = channels.bound_mask(i);
        *w = flags_word(i).map_or(0, |f| f.fetch_and(keep, Ordering::AcqRel) & !keep);
    }
    n
}

/// Whether a channel's event flag was set, clearing it.
pub fn channel_event(relid: u32) -> bool {
    let bit = 1u64 << (relid % 64);
    flags_word(relid as usize / 64).is_some_and(|f| f.fetch_and(!bit, Ordering::AcqRel) & bit != 0)
}

/// HvPostMessage: the hypercall's status, or None for a message too long.
pub(super) fn post(connection: u32, ty: u32, payload: &[u8]) -> Option<u16> {
    let s = SYNIC.get()?;
    let page = s.post.lock();
    // SAFETY: the post page, owned here and held by the lock.
    let bytes = unsafe { core::slice::from_raw_parts_mut(page.at(0), PAGE_BYTES) };
    post_input(bytes, connection, ty, payload)?;
    fence(Ordering::SeqCst);
    Some(status(call(control(CALL_POST_MESSAGE, false), page.phys, 0)))
}

/// HvSignalEvent, a fast call: the hypercall's status.
pub fn signal(connection: u32) -> Option<u16> {
    SYNIC.get()?;
    Some(status(call(control(CALL_SIGNAL_EVENT, true), signal_input(connection, 0), 0)))
}
