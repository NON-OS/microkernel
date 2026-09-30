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

//! Global inbox registry.
//!
//! Production routing rule: every inbox has an owner pid, recorded
//! at registration. `try_enqueue_strict` fails with `MissingInbox`
//! if no row exists, with `DeadOwner` if the owner pid has fallen
//! out of `PROCESS_TABLE`, and with `QueueFull` if the bounded
//! queue is full. There is no auto-registration on the send/recv
//! paths. The only path that creates an inbox without an explicit
//! pid is `register_or_get_bootstrap_inbox`, used by `capsule_spawn`
//! to set up the kernel's reply inboxes (owner = 0 = kernel).

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use spin::RwLock;

use super::error::{InboxError, StrictEnqueueError};
use super::inbox::Inbox;
use super::stats::InboxStatsSnapshot;
use crate::ipc::nonos_channel::IpcMessage;

pub const DEFAULT_INBOX_CAPACITY: usize = 1024;
pub const MIN_INBOX_CAPACITY: usize = 16;
pub const MAX_INBOX_CAPACITY: usize = 65536;

/// `0` marks an inbox owned by the kernel rather than a capsule.
/// Used for reply inboxes the spawn pipeline pre-registers; never
/// liveness-checked.
pub const KERNEL_OWNER: u32 = 0;

struct Registry {
    map: BTreeMap<String, Arc<Inbox>>,
}

impl Registry {
    const fn new() -> Self {
        Self { map: BTreeMap::new() }
    }
}

static REGISTRY: RwLock<Registry> = RwLock::new(Registry::new());
static DEFAULT_CAP: AtomicUsize = AtomicUsize::new(DEFAULT_INBOX_CAPACITY);
static GLOBAL_STATS: GlobalStats = GlobalStats::new();

struct GlobalStats {
    total_inboxes_created: AtomicU64,
    total_inboxes_removed: AtomicU64,
}

impl GlobalStats {
    const fn new() -> Self {
        Self { total_inboxes_created: AtomicU64::new(0), total_inboxes_removed: AtomicU64::new(0) }
    }
}

pub fn set_default_capacity(cap: usize) {
    let clamped = cap.clamp(MIN_INBOX_CAPACITY, MAX_INBOX_CAPACITY);
    DEFAULT_CAP.store(clamped, Ordering::Relaxed);
}

pub fn get_default_capacity() -> usize {
    DEFAULT_CAP.load(Ordering::Relaxed)
}

/// Register an inbox owned by `owner_pid`. Errors if the name is
/// empty or already registered. The default capacity applies; use
/// [`register_inbox_with_capacity`] for a custom bound.
pub fn register_inbox(module: &str, owner_pid: u32) -> Result<(), InboxError> {
    register_inbox_with_capacity(module, owner_pid, DEFAULT_CAP.load(Ordering::Relaxed))
}

/// Register an inbox with an explicit capacity and owner pid.
pub fn register_inbox_with_capacity(
    module: &str,
    owner_pid: u32,
    capacity: usize,
) -> Result<(), InboxError> {
    if module.is_empty() {
        return Err(InboxError::EmptyModuleName);
    }
    if !(MIN_INBOX_CAPACITY..=MAX_INBOX_CAPACITY).contains(&capacity) {
        return Err(InboxError::InvalidCapacity {
            value: capacity,
            min: MIN_INBOX_CAPACITY,
            max: MAX_INBOX_CAPACITY,
        });
    }
    let mut reg = REGISTRY.write();
    if reg.map.contains_key(module) {
        return Err(InboxError::AlreadyRegistered { module: module.into() });
    }
    reg.map.insert(module.into(), Arc::new(Inbox::new(capacity, owner_pid)));
    GLOBAL_STATS.total_inboxes_created.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

/// Bootstrap-only: idempotently ensure a kernel-owned inbox exists.
/// Used by the spawn pipeline to register reply inboxes the kernel
/// itself will drain. Owner is `KERNEL_OWNER`; never liveness-checked.
/// Must NOT be called from normal IPC send/recv paths.
pub fn register_or_get_bootstrap_inbox(module: &str) {
    if module.is_empty() {
        return;
    }
    let mut reg = REGISTRY.write();
    if !reg.map.contains_key(module) {
        let cap = DEFAULT_CAP.load(Ordering::Relaxed);
        reg.map.insert(module.into(), Arc::new(Inbox::new(cap, KERNEL_OWNER)));
        GLOBAL_STATS.total_inboxes_created.fetch_add(1, Ordering::Relaxed);
    }
}

/// Unregister by name, dropping all queued messages. Returns the
/// dropped count, or `None` if the inbox was not registered.
pub fn unregister_inbox(module: &str) -> Option<usize> {
    let mut reg = REGISTRY.write();
    if let Some(inbox) = reg.map.remove(module) {
        GLOBAL_STATS.total_inboxes_removed.fetch_add(1, Ordering::Relaxed);
        Some(inbox.len())
    } else {
        None
    }
}

/// Drop the canonical per-process inboxes `proc.{pid}` and `stdin.{pid}`
/// for a dying capsule, zeroing what is still queued in them: a child's
/// output and its input can be someone's private text. Called from
/// `process::exit::teardown`. Returns the count dropped from `proc.{pid}`.
/// Reply inboxes (`endpoint.<u64>`) are kernel-owned and intentionally left
/// alone so a respawn reuses them; stale replies are filtered by the
/// transport's generation re-check.
pub fn unregister_for_pid(pid: u32) -> Option<usize> {
    let _ = unregister_stdin_for_pid(pid);
    remove_zeroed(&alloc::format!("proc.{}", pid))
}

/// Drop `stdin.{pid}` alone, zeroing what is queued in it. Nothing reads a
/// dead process's input, even while its output is kept for its parent.
pub fn unregister_stdin_for_pid(pid: u32) -> Option<usize> {
    remove_zeroed(&alloc::format!("stdin.{}", pid))
}

fn remove_zeroed(module: &str) -> Option<usize> {
    let inbox = REGISTRY.write().map.remove(module)?;
    GLOBAL_STATS.total_inboxes_removed.fetch_add(1, Ordering::Relaxed);
    let mut dropped = 0;
    while let Some(mut msg) = inbox.dequeue() {
        crate::crypto::secure_zero(&mut msg.data);
        dropped += 1;
    }
    Some(dropped)
}

/// Take at most `max` bytes from the front of `module`, as a stream: the
/// rest of a longer message stays first in line for the next take, so no
/// byte is dropped. The kernel's copies of what was split are zeroed. The
/// registry is held for writing throughout, so no other reader or writer
/// sees the queue while it is put back together. `None` when the inbox is
/// missing or empty.
pub fn take_front(module: &str, max: usize) -> Option<Vec<u8>> {
    let reg = REGISTRY.write();
    let inbox = reg.map.get(module)?;
    let mut msg = inbox.dequeue()?;
    if msg.data.len() <= max {
        return Some(core::mem::take(&mut msg.data));
    }
    let head = msg.data[..max].to_vec();
    let rest = IpcMessage::with_timestamp(&msg.from, &msg.to, &msg.data[max..], msg.timestamp_ms);
    crate::crypto::secure_zero(&mut msg.data);
    let mut queued = Vec::new();
    while let Some(m) = inbox.dequeue() {
        queued.push(m);
    }
    /*
     * The queue held `msg` and `queued`, so the remainder and `queued` fit
     * back in. A remainder is only unbuildable without the IPC secret, which
     * the message itself was built with; should it fail, say so.
     */
    if rest.is_err() {
        crate::sys::serial::print(b"[INBOX] remainder of a split message lost\n");
    }
    for m in rest.ok().into_iter().chain(queued) {
        if let Err(mut lost) = inbox.try_enqueue(m) {
            crate::crypto::secure_zero(&mut lost.data);
        }
    }
    Some(head)
}

/// Strict enqueue. The inbox must exist; if its owner is not
/// `KERNEL_OWNER`, that pid must still be in `PROCESS_TABLE`. No
/// auto-registration. The owner liveness check covers the race
/// where exit teardown unregisters the endpoint+inbox between a
/// caller's `lookup_service` and the enqueue.
pub fn try_enqueue_strict(module: &str, msg: IpcMessage) -> Result<(), StrictEnqueueError> {
    let reg = REGISTRY.read();
    let inbox = reg.map.get(module).ok_or(StrictEnqueueError::MissingInbox)?;
    let owner = inbox.owner();
    if owner != KERNEL_OWNER && crate::process::get_process_table().find_by_pid(owner).is_none() {
        return Err(StrictEnqueueError::DeadOwner);
    }
    inbox.try_enqueue(msg).map_err(StrictEnqueueError::QueueFull)
}

/// Dequeue without auto-registration. Returns `None` if no inbox is
/// registered under that name. The dequeuing capsule must have been
/// pre-registered (the kernel for reply inboxes, `capsule_spawn` for
/// `proc.{pid}`).
pub fn try_dequeue_existing(module: &str) -> Option<IpcMessage> {
    let reg = REGISTRY.read();
    reg.map.get(module).and_then(|inbox| inbox.dequeue())
}

pub fn peek(module: &str) -> Option<IpcMessage> {
    REGISTRY.read().map.get(module).and_then(|i| i.peek())
}

pub fn len(module: &str) -> usize {
    REGISTRY.read().map.get(module).map(|i| i.len()).unwrap_or(0)
}

pub fn is_full(module: &str) -> bool {
    REGISTRY.read().map.get(module).map(|i| i.is_full()).unwrap_or(false)
}

pub fn is_empty(module: &str) -> bool {
    REGISTRY.read().map.get(module).map(|i| i.is_empty()).unwrap_or(true)
}

pub fn capacity(module: &str) -> Option<usize> {
    REGISTRY.read().map.get(module).map(|i| i.capacity())
}

pub fn exists(module: &str) -> bool {
    REGISTRY.read().map.contains_key(module)
}

pub fn get_inbox_stats(module: &str) -> Option<InboxStatsSnapshot> {
    REGISTRY.read().map.get(module).map(|i| i.get_stats())
}

pub fn clear(module: &str) -> usize {
    REGISTRY.read().map.get(module).map(|i| i.clear()).unwrap_or(0)
}

pub fn list_inboxes() -> Vec<String> {
    REGISTRY.read().map.keys().cloned().collect()
}

pub fn inbox_count() -> usize {
    REGISTRY.read().map.len()
}

pub fn get_global_stats() -> (u64, u64) {
    (
        GLOBAL_STATS.total_inboxes_created.load(Ordering::Relaxed),
        GLOBAL_STATS.total_inboxes_removed.load(Ordering::Relaxed),
    )
}
