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

//! A guest's pixels pinned in place as a surface the compositor reads, so a
//! Linux window is shown from the client's own buffer with no copy.

use alloc::vec::Vec;

use crate::hardware::broker::holder_of;
use crate::kernel_core::surface_registry::pin::pin_guest::{pin_guest, unpin_guest};
use crate::kernel_core::surface_registry::pin::pin_rule::{
    may_read, pin_frames, pin_pages, GuestPage, PinRefusal, PinShape,
};
use crate::kernel_core::surface_registry::types::FMT_ARGB8888;
use crate::kernel_core::surface_registry::{
    RegistryError, SurfaceDescriptor, MAX_PAGES_PER_SURFACE,
};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::page_in_asid;
use crate::memory::paging::types::PagePermissions;
use crate::syscall::microkernel::errnos::{
    ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM, ERRNO_SRCH,
};

use super::peer_guard::{pid_arg, supervised_asid, PAGE, USER_VA_END};

/// `MkPeerPin`: pin the pages of guest `pid` that `desc` names (its
/// `base_va` a guest address) as a surface the guest's group owns, shared
/// read-only with `reader` and, when not zero, with `scanout`. Returns the
/// surface's handle. Refused unless the caller supervises the guest, every
/// page is mapped for the guest and none executable, and no frame is in
/// another surface or a device's reach.
pub fn sys_peer_pin(pid: u64, desc_ptr: u64, reader: u64, scanout: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let (asid, _held) = match supervised_asid(caller, pid) {
        Ok(pair) => pair,
        Err(e) => return e,
    };
    let guest = match pid_arg(pid) {
        Ok(g) => g,
        Err(e) => return e,
    };
    let Ok(asked) = crate::usercopy::read_user_value::<SurfaceDescriptor>(desc_ptr) else {
        return ERRNO_FAULT;
    };
    let Some(desc) = descriptor(&asked) else {
        return ERRNO_INVAL;
    };
    let readers = match readers(reader, scanout) {
        Ok(r) => r,
        Err(refused) => return errno_of(refused),
    };
    let shape = PinShape {
        addr: desc.base_va,
        byte_len: desc.byte_len,
        width: desc.width,
        height: desc.height,
        stride: desc.stride,
    };
    let pages = match pin_pages(shape, USER_VA_END, MAX_PAGES_PER_SURFACE) {
        Ok(n) => n,
        Err(refused) => return errno_of(refused),
    };
    let walked: Vec<Option<GuestPage>> =
        (0..pages as u64).map(|i| guest_page(asid, desc.base_va + i * PAGE)).collect();
    let frames = match pin_frames(&walked) {
        Ok(f) => f.into_iter().map(PhysAddr::new).collect(),
        Err(refused) => return errno_of(refused),
    };
    let (group, supervisor) = (holder_of(guest), holder_of(caller));
    match pin_guest((guest, group, supervisor), &desc, frames, &readers) {
        Ok(handle) => handle as i64,
        Err(e) => errno_of_registry(e),
    }
}

/// `MkPeerUnpin`: give up the surface `handle` a pin of guest `pid` made.
/// No process attaches it again; a reader's view already mapped keeps its
/// frames until that reader lets go.
pub fn sys_peer_unpin(pid: u64, handle: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let (_asid, _held) = match supervised_asid(caller, pid) {
        Ok(pair) => pair,
        Err(e) => return e,
    };
    let guest = match pid_arg(pid) {
        Ok(g) => g,
        Err(e) => return e,
    };
    let group = holder_of(guest);
    match unpin_guest(group, handle) {
        Ok(()) => 0,
        Err(e) => errno_of_registry(e),
    }
}

/// The descriptor a pin registers: only the shape the caller asked for, in
/// the one format a surface has, with no flags.
fn descriptor(asked: &SurfaceDescriptor) -> Option<SurfaceDescriptor> {
    if asked.format != FMT_ARGB8888 || asked.flags != 0 {
        return None;
    }
    Some(SurfaceDescriptor {
        width: asked.width,
        height: asked.height,
        stride: asked.stride,
        format: FMT_ARGB8888,
        byte_len: asked.byte_len,
        base_va: asked.base_va,
        flags: 0,
        owner_pid: 0,
        writable: 0,
    })
}

/// The pids named to read the surface: `reader`, the compositor, always;
/// `scanout`, a display driver, when not zero (pin_rule.rs may_read). Each
/// is checked live and not a guest when it is named (share_surface).
fn readers(reader: u64, scanout: u64) -> Result<Vec<u32>, PinRefusal> {
    let pid = |raw: u64| u32::try_from(raw).ok().filter(|&p| p != 0).ok_or(PinRefusal::Reader);
    let reader = pid(reader)?;
    if !serves(reader, false) {
        return Err(PinRefusal::Reader);
    }
    let mut named = alloc::vec![reader];
    if scanout != 0 {
        let scanout = pid(scanout)?;
        if !serves(scanout, true) {
            return Err(PinRefusal::Reader);
        }
        named.push(scanout);
    }
    Ok(named)
}

/// Whether `pid` serves the compositor's endpoint, or a display driver's
/// when `scanout`.
fn serves(pid: u32, scanout: bool) -> bool {
    let served = crate::services::registry::endpoints_of(pid);
    let names: Vec<&str> = served.iter().map(|e| e.name.as_str()).collect();
    may_read(&names, crate::services::registry::displays::DISPLAYS, scanout)
}

fn guest_page(asid: u32, va: u64) -> Option<GuestPage> {
    let (frame, perms) = page_in_asid(asid, VirtAddr::new(va))?;
    Some(GuestPage {
        frame: frame.as_u64() & !(PAGE - 1),
        user: perms.contains(PagePermissions::USER),
        exec: perms.contains(PagePermissions::EXECUTE),
    })
}

fn errno_of(refused: PinRefusal) -> i64 {
    match refused {
        PinRefusal::Shape | PinRefusal::Unmapped => ERRNO_INVAL,
        PinRefusal::Executable
        | PinRefusal::DeviceWindow
        | PinRefusal::AlreadyHeld
        | PinRefusal::Reader => ERRNO_PERM,
        PinRefusal::Full => ERRNO_NOMEM,
    }
}

fn errno_of_registry(e: RegistryError) -> i64 {
    match e {
        RegistryError::InvalidArg | RegistryError::BadHandle | RegistryError::NotFound => {
            ERRNO_INVAL
        }
        RegistryError::NotOwner => ERRNO_PERM,
        RegistryError::OutOfSlots | RegistryError::MapFailed => ERRNO_NOMEM,
        RegistryError::NoProc => ERRNO_SRCH,
    }
}
