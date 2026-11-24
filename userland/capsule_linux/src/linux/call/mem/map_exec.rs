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

//! Proving a file before any of its pages become executable.

use alloc::vec::Vec;

use crate::linux::file::{key, store_read};
use crate::linux::guest::{Guest, Kind};

use super::map_whole::whole;

/// The same ceiling the exec path reads an image under.
const MAX_IMAGE: u32 = crate::linux::program_max::PROGRAM_MAX;

/// The bytes of `fd`'s file, if this machine has agreed to execute them. The
/// mapping is filled from these, not read again: a second read could see a
/// file rewritten after it was proved, and map those bytes executable. They
/// come from the descriptor's own stream when it can give them whole
/// (map_whole.rs), so a library the loader has just read is not read again,
/// and from the store otherwise; either way they are proved as they are.
pub fn proven(guest: &mut Guest, fd: u64) -> Option<Vec<u8>> {
    let entry = guest.fds.get_mut(fd as usize).filter(|f| f.kind == Kind::File)?;
    /*
     * A descriptor's path was normalised when it was opened, so it
     * needs no resolving here, only confining.
     */
    let at = entry.path.clone();
    let bytes = match whole(entry, MAX_IMAGE) {
        Some(bytes) => bytes,
        None => store_read(&key(&at), MAX_IMAGE).ok()?,
    };
    crate::linux::attest::verify(&at, &bytes).ok()?;
    crate::linux::start_time::mapped();
    Some(bytes)
}
