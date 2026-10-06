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

//! The tid `clone` writes where its caller asked (CLONE_PARENT_SETTID and
//! CLONE_CHILD_SETTID) is the number the guest sees, the same one clone
//! returns. musl keeps the parent's copy as the thread's own tid and hands it
//! to tkill, so the kernel's pid written there named no thread of the guest.

use nonos_libc::ForeignFrame;

use crate::linux::abi::nr;
use crate::linux::guest::Guest;

const CLONE_PARENT_SETTID: u64 = 0x10_0000;
const CLONE_CHILD_SETTID: u64 = 0x100_0000;

/// After a clone that made a thread, write its guest-side `tid` where the
/// flags ask. Linux ignores a word it cannot write.
pub(super) fn write(guest: &Guest, frame: &ForeignFrame, tid: u64) {
    if frame.nr != nr::CLONE || tid as i64 <= 0 {
        return;
    }
    let a = frame.args();
    let word = (tid as u32).to_le_bytes();
    if a[0] & CLONE_PARENT_SETTID != 0 {
        let _ = guest.write(a[2], &word);
    }
    if a[0] & CLONE_CHILD_SETTID != 0 {
        let _ = guest.write(a[3], &word);
    }
}
