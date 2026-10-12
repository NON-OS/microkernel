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

/// The oldest message taken, header and payload, and its length.
pub fn recv(out: &mut [u8; MESSAGE_BYTES]) -> Option<usize> {
    service();
    let m = RING.lock().pop()?;
    let Slot::Full { len, .. } = read(&m) else { return None };
    *out = m;
    Some(HEADER_BYTES + len)
}

/// VMBus's event flags, each word handed over and cleared at once.
pub fn events(out: &mut [u64]) -> usize {
    let Some(s) = SYNIC.get() else { return 0 };
    let base = s.siefp.at(SINT_VMBUS as usize * MESSAGE_BYTES) as *const AtomicU64;
    let n = out.len().min(EVENT_WORDS);
    for (i, w) in out.iter_mut().take(n).enumerate() {
        // SAFETY: word i of VMBus's 256 bytes in the event flags page, which
        // the hypervisor sets bits in atomically and this swap clears.
        *w = unsafe { &*base.add(i) }.swap(0, Ordering::AcqRel);
    }
    n
}

/// HvPostMessage: the hypercall's status, or None for a message too long.
pub fn post(connection: u32, ty: u32, payload: &[u8]) -> Option<u16> {
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
