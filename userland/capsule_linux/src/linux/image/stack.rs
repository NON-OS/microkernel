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

//! The stack a Linux program expects to wake up on: argc, then argv, then
//! the environment, then the auxiliary vector, each list ended by a null.
//! A C runtime reads all four before `main` and crashes without them.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

pub const AT_NULL: u64 = 0;
pub const AT_PAGESZ: u64 = 6;
pub const AT_ENTRY: u64 = 9;
pub const AT_UID: u64 = 11;
pub const AT_EUID: u64 = 12;
pub const AT_GID: u64 = 13;
pub const AT_EGID: u64 = 14;

/// Build the initial stack in `guest` and return the `rsp` it starts on.
/// The layout is fixed by the System V supplement, not by us.
pub fn build(guest: &Guest, top: u64, entry: u64) -> Option<u64> {
    let mut words: Vec<u64> = Vec::new();
    /*
     * One argument, no environment. The strings go above the vector and
     * are pointed at from it, so the whole block is written once and the
     * guest sees a stack indistinguishable from the one Linux builds.
     */
    let name = b"guest\0";
    let name_at = top - name.len() as u64;
    words.push(1);
    words.push(name_at);
    words.push(0);
    words.push(0);
    for (key, value) in
        [(AT_PAGESZ, 4096), (AT_ENTRY, entry), (AT_UID, 0), (AT_EUID, 0), (AT_GID, 0), (AT_EGID, 0)]
    {
        words.push(key);
        words.push(value);
    }
    words.push(AT_NULL);
    words.push(0);
    let bytes = words.len() as u64 * 8;
    let rsp = (name_at - bytes) & !0xF;
    if guest.write(name_at, name) < 0 {
        return None;
    }
    let mut blob: Vec<u8> = Vec::with_capacity(bytes as usize);
    for word in &words {
        blob.extend_from_slice(&word.to_le_bytes());
    }
    if guest.write(rsp, &blob) < 0 {
        return None;
    }
    Some(rsp)
}
