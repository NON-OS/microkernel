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


//! The descriptors in a control block, read out of the guest and walked by
//! `cmsg`, which the host proofs hold.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

/// The most control data one message carries here: room for SCM_MAX_FD
/// descriptors (a block of 1028 bytes, 1032 aligned) and a header or two
/// besides. At 1024 a block of all 253 was cut short and none was taken.
const MOST: u64 = 2048;
// A header of 16 bytes and every descriptor one message may carry fit.
const _: () = assert!(MOST as usize >= 16 + super::cmsg::MAX_FDS * 4);

/// Every descriptor in the message's SCM_RIGHTS blocks; None when the
/// control data named cannot be read (EFAULT, as Linux answers), rather
/// than a message sent as if it had passed nothing.
pub(super) fn rights(guest: &Guest, at: u64, len: u64) -> Option<Vec<u32>> {
    if len == 0 {
        return Some(Vec::new());
    }
    guest.read(at, len.min(MOST) as usize).map(|raw| super::cmsg::rights(&raw))
}
