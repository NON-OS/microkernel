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
/// descriptors and a header or two besides.
const MOST: u64 = 1024;

/// Every descriptor in the message's SCM_RIGHTS blocks, none if the control
/// data cannot be read.
pub(super) fn rights(guest: &Guest, at: u64, len: u64) -> Vec<u32> {
    match guest.read(at, len.min(MOST) as usize) {
        Some(raw) => super::cmsg::rights(&raw),
        None => Vec::new(),
    }
}
