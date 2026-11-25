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

use crate::linux::file::{key, store_read, Stream};
use crate::linux::guest::{Guest, Kind};

/// The same ceiling the exec path reads an image under.
const MAX_IMAGE: u32 = crate::linux::program_max::PROGRAM_MAX;

/// The bytes of `fd`'s file, if this machine has agreed to execute them. The
/// mapping is filled from these, not read again: a second read could see a
/// file rewritten after it was proved, and map those bytes executable.
pub fn proven(guest: &mut Guest, fd: u64) -> Option<Vec<u8>> {
    let entry = guest.fds.get_mut(fd as usize).filter(|f| f.kind == Kind::File)?;
    let bytes = match held(&mut entry.stream) {
        Some(bytes) => bytes,
        /*
         * A descriptor's path was normalised when it was opened, so it
         * needs no resolving here, only confining.
         */
        None => store_read(&key(&entry.path), MAX_IMAGE).ok()?,
    };
    crate::linux::attest::verify(&entry.path, &bytes).ok()?;
    Some(bytes)
}

/*
 * A packed file's bytes, from what its descriptor has already read and
 * checked against the pack's digests: a loader maps a library's whole span
 * before its code, so every page is usually held by then, and reading the
 * file again from the store would check the same bytes twice. The pack is
 * held open for the run, so the file cannot change under the descriptor.
 */
fn held(stream: &mut Option<Stream>) -> Option<Vec<u8>> {
    match stream.as_mut()? {
        Stream::Packed(bytes) => Some(bytes.clone()),
        Stream::Paged(paged) if paged.size <= u64::from(MAX_IMAGE) => {
            paged.read_window(0, paged.size as u32).ok()
        }
        _ => None,
    }
}
