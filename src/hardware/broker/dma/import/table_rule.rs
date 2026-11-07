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

//! The live imports, pure: who holds which, and which a drain takes.
//! kernel_proofs::surface_import.

extern crate alloc;

use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Import {
    pub id: u64,
    /// The holder (thread group) that imported it.
    pub pid: u32,
    pub device_id: u64,
    pub epoch: u64,
    pub handle: u64,
    pub iova: u64,
    pub length: u64,
    /// The surface's frames, as runs, for the quarantine.
    pub runs: Vec<(u64, u64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnimportError {
    Unknown,
    NotHolder,
}

pub(crate) fn held_by(list: &[Import], pid: u32) -> usize {
    list.iter().filter(|i| i.pid == pid).count()
}

/// The holder's own import `id`, out of the list.
pub(crate) fn take_one(list: &mut Vec<Import>, pid: u32, id: u64) -> Result<Import, UnimportError> {
    let at = list.iter().position(|i| i.id == id).ok_or(UnimportError::Unknown)?;
    if list[at].pid != pid {
        return Err(UnimportError::NotHolder);
    }
    Ok(list.remove(at))
}

/// Every import `pick` names, out of the list, oldest first.
pub(crate) fn take_where(list: &mut Vec<Import>, pick: impl Fn(&Import) -> bool) -> Vec<Import> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < list.len() {
        if pick(&list[i]) {
            out.push(list.remove(i));
        } else {
            i += 1;
        }
    }
    out
}
