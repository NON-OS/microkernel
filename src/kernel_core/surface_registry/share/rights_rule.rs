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

//! Who may attach a surface, pure; kernel_proofs::surface_closing::
//! rights. A share names one receiver, recorded here; rights_attach.rs says
//! who attaches.

extern crate alloc;

use alloc::vec::Vec;

/// Receivers one surface names at once; one more is ENOMEM.
pub(crate) const MAX_RECEIVERS: usize = 4;
/// MkSurfaceShare flag: the receiver maps the frames writable.
pub(crate) const SHARE_WRITE: u64 = 1;

/// Whether the flags ask for write; None for a bit no flag names.
pub(crate) const fn share_write(flags: u64) -> Option<bool> {
    if flags & !SHARE_WRITE != 0 {
        None
    } else {
        Some(flags & SHARE_WRITE != 0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Right {
    pub handle: u64,
    pub pid: u32,
    pub write: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Grant {
    New,
    Already,
    Full,
}

/// Record `right`; naming a receiver again only widens its write.
pub(crate) fn grant(rights: &mut Vec<Right>, right: Right) -> Grant {
    let same = |r: &&mut Right| r.handle == right.handle && r.pid == right.pid;
    if let Some(r) = rights.iter_mut().find(same) {
        r.write |= right.write;
        return Grant::Already;
    }
    if rights.iter().filter(|r| r.handle == right.handle).count() >= MAX_RECEIVERS {
        return Grant::Full;
    }
    rights.push(right);
    Grant::New
}

/// The owner released it, or it is gone: no receiver attaches again.
pub(crate) fn revoke_handle(rights: &mut Vec<Right>, handle: u64) {
    rights.retain(|r| r.handle != handle);
}

/// `pid` exited: a later process given its pid inherits nothing.
pub(crate) fn revoke_pid(rights: &mut Vec<Right>, pid: u32) {
    rights.retain(|r| r.pid != pid);
}
