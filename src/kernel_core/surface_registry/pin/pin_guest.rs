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

//! A guest's frames pinned as a surface its group owns, and given up.

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::attach_map;
use crate::kernel_core::surface_registry::share::{revoke, share_surface};
use crate::kernel_core::surface_registry::table::{register_surface, SLOTS};
use crate::kernel_core::surface_registry::types::{
    decode_handle, encode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle,
};
use crate::memory::addr::PhysAddr;
use crate::process::ProcessState;
use crate::smp::lock_responsive;

use super::gate::GATE;
use super::orphans::ORPHANS;
use super::pin_rule::{in_grant, room};
use super::register_rule::may_register;

/*
 * Register `frames`, guest `pid`'s pages at desc.base_va as its tables map
 * them now, as a surface `group` owns, then name each of `readers` to attach
 * it read-only. The caller holds the guest's peer lock, so no peer call
 * changes those pages meanwhile. The checks, the slot and the guest's attach
 * record are made under the pin gate, as register_owned's are: an exit, an
 * attach or a second registration in between would make them stale. A guest
 * already exiting is refused there: a zombie, or a group whose last thread's
 * exit has counted its surfaces (exited.rs). Checked by the pid alone, a pin
 * between that count and the slot release that follows it was dropped with
 * no orphan, and a reader that attached it in between kept a view of frames
 * the teardown then freed.
 */
pub fn pin_guest(
    (pid, group, supervisor): (u32, u32, u32),
    desc: &SurfaceDescriptor,
    frames: Vec<PhysAddr>,
    readers: &[u32],
) -> Result<SurfaceHandle, RegistryError> {
    let handle = {
        let _gate = lock_responsive(&GATE);
        if !alive(pid) || super::exited::has(group) {
            return Err(RegistryError::NoProc);
        }
        let mut new: Vec<u64> = frames.iter().map(|f| f.as_u64()).collect();
        new.sort_unstable();
        let device = crate::hardware::broker::touches_device_window(desc.base_va, desc.byte_len)
            || in_grant(&new, dma_runs());
        if !room(pinned_by(supervisor)) {
            return Err(RegistryError::OutOfSlots);
        }
        {
            let slots = SLOTS.lock();
            let orphans = ORPHANS.lock();
            let live = slots.iter().flatten().flat_map(|s| s.frames.iter());
            let orphaned = orphans.iter().flat_map(|o| o.frames.iter());
            may_register(device, &new, live.chain(orphaned).map(|f| f.as_u64()))
                .map_err(|_| RegistryError::NotOwner)?;
        }
        let (_, handle) = register_surface(group, desc, frames, None)?;
        attach_map::record(group, handle, desc.base_va, desc.byte_len);
        handle
    };
    for &reader in readers {
        if let Err(e) = share_surface(group, handle, reader, false) {
            let _ = unpin_guest(group, handle);
            return Err(e);
        }
    }
    Ok(handle)
}

/*
 * `group` gives up the surface `handle` names: no process attaches it again
 * from now, and the guest's own reference and record go. A view a reader
 * already maps keeps the slot and its frames until that reader lets go, and
 * the guest's unmap or exit then leaves those frames to the registry, as an
 * owner's does (claim_owned, exit_owned), so no frame a reader maps is freed
 * under it.
 */
pub fn unpin_guest(group: u32, handle: SurfaceHandle) -> Result<(), RegistryError> {
    {
        let _gate = lock_responsive(&GATE);
        if !owns(group, handle) {
            return Err(RegistryError::NotFound);
        }
        revoke(handle);
    }
    super::drop_attach::drop_attach(group, handle).map(|_| ())
}

/// Unpin every surface `group` holds that meets `[addr, addr + len)`, before
/// a peer call changes those pages.
pub fn unpin_meeting(group: u32, addr: u64, len: u64) {
    let end = addr.saturating_add(len);
    let meeting: Vec<SurfaceHandle> = SLOTS
        .lock()
        .iter()
        .enumerate()
        .filter_map(|(idx, e)| {
            let s = e.as_ref()?;
            let win_end = s.owner_base_va.saturating_add(s.frames.len() as u64 * 4096);
            let meets = s.owner_base_va < end && addr < win_end;
            (s.owner_pid == group && meets).then(|| encode_handle(idx as u32, s.epoch))
        })
        .collect();
    for handle in meeting {
        let _ = unpin_guest(group, handle);
    }
}

/// Whether `handle` names a live slot `group` owns, with the guest's own
/// record still on it.
fn owns(group: u32, handle: SurfaceHandle) -> bool {
    let (idx, epoch) = decode_handle(handle);
    let live = SLOTS
        .lock()
        .get(idx as usize)
        .and_then(|s| s.as_ref())
        .is_some_and(|s| s.epoch == epoch && s.owner_pid == group);
    live && attach_map::lookup(group, handle).is_some()
}

/// How many slots the guests `supervisor` hosts own now. Under the gate,
/// as every registration is; the slots' owners are read first and their
/// supervisors after, so the table's lock is never held over the foreign
/// registry's.
fn pinned_by(supervisor: u32) -> usize {
    let owners: Vec<u32> = SLOTS.lock().iter().flatten().map(|s| s.owner_pid).collect();
    let of = |owner: &u32| crate::process::foreign::supervisor_of(*owner);
    owners
        .iter()
        .filter(|o| of(o).map(crate::hardware::broker::holder_of) == Some(supervisor))
        .count()
}

fn alive(pid: u32) -> bool {
    crate::process::get_process(pid).is_some_and(|p| {
        !matches!(*p.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    })
}

fn dma_runs() -> Vec<(u64, u64)> {
    let grants = crate::hardware::broker::dma::grant_ranges();
    grants.into_iter().map(|(_, start, len)| (start, len)).collect()
}
