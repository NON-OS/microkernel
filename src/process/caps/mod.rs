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

//! Per-pid capability authority.
//!
//! The PCB's stored `Arc<CapabilityToken>` is the source of truth.
//! `caps_bits` is a derived bitmap cache so the cheap readers stay an
//! atomic load. Every mutator here mints a fresh token and updates
//! the cache under the same write lock through `authority::publish`.
//!
//! Token mint is fail-closed on the boot-session nonce: if the boot
//! singleton has not been initialized yet, `new_token` returns `None`
//! so no authority is silently bound to a zero nonce.
//!
//! Mutators are serialised by one lock. Each reads the bits it changes and
//! writes the result; two of them interleaved could publish a set built from
//! bits the other had already replaced (a grant reinstating what a revoke had
//! just removed). The gate never takes this lock.

pub(crate) mod authority;
pub(crate) mod epoch;
pub(crate) mod mint;
pub(crate) mod reissue;

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

use crate::capabilities::CapabilityToken;
use crate::process::core::ProcessControlBlock;

use super::api::with_process;
use authority::publish;
use mint::{mint, Binding};
use reissue::{reissue, Change, Member, Refusal};

/*
 * Lock order: taken under `address_space::lifecycle::holder`'s RELEASE (a
 * thread's exit handing its ASID to a sibling rebinds the heir), and takes,
 * while held, the process table's read lock (`group`, `with_process`), the
 * paging manager (`lookup_asid_for_process`), the boot session cell and a
 * PCB's token and allowlist write locks. So nothing holding the process
 * table, the paging manager or a token lock may call grant, revoke,
 * install_spawn, join_group or rebind_address_space. The call sites are
 * listed, each with what it holds, in kernel_proofs
 * (`caps_lock_order_tests.rs`); a new one fails there until reviewed.
 */
static MUTATE: spin::Mutex<()> = spin::Mutex::new(());

fn binding(pid: u32, epoch: u64) -> Option<Binding> {
    Some(Binding {
        pid,
        asid: crate::memory::paging::manager::lookup_asid_for_process(pid).unwrap_or(0),
        epoch,
        boot_nonce: crate::security::boot_session::nonce()?,
    })
}

pub(crate) fn new_token(pid: u32, bits: u64) -> Option<Arc<CapabilityToken>> {
    let epoch = with_process(pid, |pcb| pcb.revocation_epoch.load(Ordering::Acquire)).unwrap_or(0);
    mint(bits, binding(pid, epoch)?)
}

fn member(pcb: &ProcessControlBlock) -> Option<Member<'_>> {
    Some(Member { cells: pcb.authority_cells(), binding: binding(pcb.pid, 0)? })
}

/// The PCBs of `pid`'s thread group, `pid`'s own first.
fn group(pid: u32) -> Option<Vec<Arc<ProcessControlBlock>>> {
    let target = crate::process::core::PROCESS_TABLE.find_by_pid(pid)?;
    let tgid = target.thread_group_id();
    let mut all: Vec<_> = crate::process::core::PROCESS_TABLE
        .get_all_processes()
        .into_iter()
        .filter(|p| p.pid != pid && p.thread_group_id() == tgid)
        .collect();
    all.insert(0, target);
    Some(all)
}

fn apply(pcbs: &[Arc<ProcessControlBlock>], from: u64, change: Change) -> Result<(), Refusal> {
    let members: Option<Vec<Member>> = pcbs.iter().map(|p| member(p)).collect();
    reissue(&members.ok_or(Refusal::Unsigned)?, from, change)
}

/// Refresh the stored token after the PCB's address space has been
/// established. Constructors call this after `lifecycle::allocate` or
/// `lifecycle::inherit` so `subject_asid` reflects the real value
/// before the PCB is reachable through the process table.
pub(crate) fn rebind_address_space(pcb: &ProcessControlBlock) -> Option<()> {
    let _held = MUTATE.lock();
    let bits = pcb.caps_bits.load(Ordering::Acquire);
    let fresh = new_token(pcb.pid, bits)?;
    publish(&pcb.authority_cells(), fresh);
    Some(())
}

/*
 * A new thread copies its process's bits before it is in the table, so a
 * grant or revoke of the group in that window does not reach it. Once it
 * is in the table and before it can run, it takes the bits the group holds
 * now; any later change finds it in the table.
 */
pub(crate) fn join_group(pcb: &Arc<ProcessControlBlock>) -> Option<()> {
    let _held = MUTATE.lock();
    let tgid = pcb.thread_group_id();
    let peers = crate::process::core::PROCESS_TABLE.get_all_processes();
    let peer = peers
        .iter()
        .filter(|p| p.pid != pcb.pid && p.thread_group_id() == tgid)
        .min_by_key(|p| p.pid != tgid)?;
    let bits = peer.caps_bits.load(Ordering::Acquire);
    if bits == pcb.caps_bits.load(Ordering::Acquire) {
        return Some(());
    }
    apply(core::slice::from_ref(pcb), bits, Change::Set(bits)).ok()
}

pub fn bits(pid: u32) -> Option<u64> {
    with_process(pid, |pcb| pcb.caps_bits.load(Ordering::Acquire))
}

/// Checks whether `pid` holds every bit in `mask`. Fail-closed on an
/// unknown pid.
pub fn has(pid: u32, mask: u64) -> bool {
    bits(pid).map(|b| (b & mask) == mask).unwrap_or(false)
}

/// Caller is responsible for any authority check before reaching here.
/// Reaches every thread of `pid`'s process.
pub fn grant(pid: u32, mask: u64) -> Option<()> {
    let _held = MUTATE.lock();
    let pcbs = group(pid)?;
    let from = pcbs[0].caps_bits.load(Ordering::Acquire);
    apply(&pcbs, from, Change::Grant(mask)).ok()?;
    crate::log::info!("[CAPS] pid={} granted {:#x}: {} thread(s) reissued", pid, mask, pcbs.len());
    Some(())
}

/// Reaches every thread of `pid`'s process; each thread's next syscall is
/// decided on the reduced set.
pub fn revoke(pid: u32, mask: u64) -> Option<()> {
    let _held = MUTATE.lock();
    let pcbs = group(pid)?;
    let from = pcbs[0].caps_bits.load(Ordering::Acquire);
    match apply(&pcbs, from, Change::Revoke(mask)) {
        Ok(()) => {
            crate::log::info!("[CAPS] pid={} revoked {:#x}: {} thread(s) reissued", pid, mask, pcbs.len());
            Some(())
        }
        Err(Refusal::EpochExhausted) => {
            crate::log::log_error!(
                "[CAPS] pid={} revocation epoch exhausted: revoke refused, every capability withdrawn",
                pid
            );
            None
        }
        Err(Refusal::Unsigned) => None,
    }
}

/// The bits `pid`'s signed manifest declares, for the deny path.
pub fn declare(pid: u32, envelope: u64) -> Option<()> {
    with_process(pid, |pcb| pcb.declared_caps.store(envelope, Ordering::Release))
}

/// Record that the spawn gate withheld Debug from `pid`, for `MkProcStat`.
pub fn mark_debug_withheld(pid: u32) {
    with_process(pid, |pcb| pcb.debug_withheld.store(true, Ordering::Relaxed));
}

/// One-shot manifest install. Replaces whatever inheritance-derived
/// token the PCB was born with. A second call is rejected so a stale
/// spawn path cannot re-issue authority.
pub fn install_spawn(pid: u32, mask: u64) -> Option<()> {
    let _held = MUTATE.lock();
    with_process(pid, |pcb| {
        let fresh = new_token(pcb.pid, mask)?;
        pcb.caps_manifest_installed
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .ok()
            .map(|_| publish(&pcb.authority_cells(), fresh))
    })
    .flatten()
}
