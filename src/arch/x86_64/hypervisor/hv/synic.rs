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

use super::channels::{BindError, Channel, Channels, Interface, Retire, Route, MAX_GPADLS};
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
use super::vmbus_out::{check, check_channel, Refusal, MESSAGE_TYPE_CHANNEL};
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

/// Broker work a host message left: records retired or settled. Taken by
/// whichever syscall looks next, in process context.
static RETIRES: Mutex<[Retire; 16]> = Mutex::new([Retire::None; 16]);

fn note(retire: Retire) {
    if retire == Retire::None {
        return;
    }
    let mut r = RETIRES.lock();
    // An unload stands for everything; otherwise the first free slot. Full,
    // the oldest settle is dropped first: a settle not taken leaves pages
    // kept, never freed early.
    let free = r.iter().position(|x| *x == Retire::None);
    let at = free.or_else(|| r.iter().position(|x| matches!(x, Retire::Settle(_))));
    match (retire, at) {
        (Retire::All, _) => *r = [Retire::None; 16],
        (_, Some(i)) => r[i] = retire,
        (_, None) => {}
    }
    if retire == Retire::All {
        r[0] = Retire::All;
    }
}

/// The broker work waiting, taken.
pub fn retires() -> [Retire; 16] {
    core::mem::replace(&mut *RETIRES.lock(), [Retire::None; 16])
}

/*
 * Move what the host left in VMBus's slot on, freeing the slot and, when
 * the host has more queued, ending the message so it sends the next. A
 * channel message updates the kernel's table of channels; a bound
 * channel's answer goes to its queue, anything else to the core's ring.
 * Only on VP 0, whose slot it is and whose EOM counts; the gate's handler
 * runs there, and so does any poll on a one-CPU guest. Then whatever the
 * kernel owes the host for a released channel is sent.
 */
pub fn service() {
    let Some(s) = SYNIC.get() else { return };
    if rdmsr(MSR_VP_INDEX) == 0 {
        let slot = s.simp.at(SINT_VMBUS as usize * MESSAGE_BYTES) as *mut [u8; MESSAGE_BYTES];
        let mut ring = RING.lock();
        loop {
            // SAFETY: VMBus's slot in the message page, which the hypervisor
            // writes and only this function, under the ring's lock, clears.
            let m = unsafe { read_volatile(slot) };
            let Slot::Full { ty, len, pending } = read(&m) else { break };
            if ring.full() {
                break;
            }
            let route = if ty == MESSAGE_TYPE_CHANNEL {
                let (route, retire) =
                    CHANNELS.lock().take_host_message(&m[HEADER_BYTES..HEADER_BYTES + len]);
                note(retire);
                route
            } else {
                Route::Core
            };
            if route == Route::Core {
                ring.push(&m);
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
    clean_up();
}

/// The oldest message for the core, header and payload, and its length.
pub fn recv(out: &mut [u8; MESSAGE_BYTES]) -> Option<usize> {
    service();
    let m = RING.lock().pop()?;
    let Slot::Full { len, .. } = read(&m) else { return None };
    *out = m;
    Some(HEADER_BYTES + len)
}

/// The oldest host answer for a record's channel, as a SynIC message, and
/// its length.
pub fn recv_channel(record: u64, out: &mut [u8; MESSAGE_BYTES]) -> Option<usize> {
    service();
    let (n, b) = CHANNELS.lock().take(record)?;
    *out = [0; MESSAGE_BYTES];
    out[..4].copy_from_slice(&MESSAGE_TYPE_CHANNEL.to_le_bytes());
    out[4] = n as u8;
    out[HEADER_BYTES..HEADER_BYTES + n].copy_from_slice(&b[..n]);
    Some(HEADER_BYTES + n)
}

/// Why a post did not go.
pub enum PostError {
    Refused(Refusal),
    /// The hypercall's status.
    Status(u16),
    NoSynic,
}

fn sent(status: Option<u16>) -> Result<(), PostError> {
    match status {
        None => Err(PostError::NoSynic),
        Some(super::hypercall_abi::STATUS_SUCCESS) => Ok(()),
        Some(s) => Err(PostError::Status(s)),
    }
}

/// A post from the VMBus core: the bus's messages alone, its pages on the
/// root. The request for offers names the bus's connection.
pub fn post_core(
    connection: u32,
    ty: u32,
    payload: &[u8],
    held: impl Fn(u64) -> bool,
) -> Result<(), PostError> {
    check(connection, ty, payload, held).map_err(PostError::Refused)?;
    sent(post(connection, ty, payload))?;
    CHANNELS.lock().take_core_message(connection, payload);
    Ok(())
}

/// A post from a channel's driver, holding `record`: its own channel's
/// messages alone, on the bus's connection, its frames on its record. One
/// lock across the check, the post and what the post changes, so the table
/// the check read is the table the post changes.
pub fn post_channel(
    record: u64,
    ty: u32,
    payload: &[u8],
    held: impl Fn(u64) -> bool,
) -> Result<(), PostError> {
    let mut channels = CHANNELS.lock();
    check_channel(ty, payload, &channels, record, held).map_err(PostError::Refused)?;
    let Some(bus) = channels.bus() else { return Err(PostError::NoSynic) };
    sent(post(bus, ty, payload))?;
    channels.take_driver_message(payload);
    Ok(())
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

/// A channel record's grants are being drained: whether their pages must
/// be kept, because the host may still hold a GPADL over them. If so, the
/// kernel closes the channel and tears its GPADLs down itself.
pub fn released(record: u64) -> bool {
    let keep = CHANNELS.lock().release(record);
    if keep {
        clean_up();
    }
    keep
}

/// What the kernel owes the host for released channels, sent: each
/// channel's close and teardowns, or none of it marked sent, for the next
/// look to send again.
fn clean_up() {
    let mut owed = [([0u8; 16], 0usize); MAX_GPADLS + 1];
    loop {
        let mut channels = CHANNELS.lock();
        let Some((relid, n)) = channels.owed(&mut owed) else { return };
        let Some(bus) = channels.bus() else { return };
        let ok = owed[..n].iter().all(|(m, len)| {
            post(bus, MESSAGE_TYPE_CHANNEL, &m[..*len])
                == Some(super::hypercall_abi::STATUS_SUCCESS)
        });
        channels.cleaned(relid, ok);
        if !ok {
            return;
        }
    }
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
