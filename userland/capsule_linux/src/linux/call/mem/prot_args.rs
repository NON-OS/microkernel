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

//! mprotect's arguments, checked as Linux checks them before it looks at a
//! single mapping, and the one rule this personality adds: no page is ever
//! writable and executable at once. Pure, so the host proofs hold it.

use crate::linux::abi::errno;
use crate::linux::guest::{page_len, PAGE, USER_MAX};

pub const PROT_WRITE: u64 = 2;
pub const PROT_EXEC: u64 = 4;
/// PROT_READ, PROT_WRITE and PROT_EXEC together: any access at all.
pub const PROT_ANY: u64 = 7;
/// PROT_SEM, which x86 accepts and gives no meaning.
const PROT_SEM: u64 = 8;
const PROT_GROWSDOWN: u64 = 0x0100_0000;
const PROT_GROWSUP: u64 = 0x0200_0000;

/// A request for both at once.
pub fn wx_refused(prot: u64) -> bool {
    prot & PROT_WRITE != 0 && prot & PROT_EXEC != 0
}

/// The page-aligned span an mprotect changes; None for an empty one, which
/// succeeds having changed nothing; or the errno that refuses it, in Linux's
/// order: both grow bits or a misaligned address EINVAL, an end that wraps
/// ENOMEM, an unknown bit EINVAL. Then a grow bit EINVAL, a writable and
/// executable page EPERM, and an end past the guest's area ENOMEM.
pub fn prot_span(addr: u64, len: u64, prot: u64) -> Result<Option<(u64, u64)>, i64> {
    let grows = prot & (PROT_GROWSDOWN | PROT_GROWSUP);
    if grows == PROT_GROWSDOWN | PROT_GROWSUP || !addr.is_multiple_of(PAGE) {
        return Err(errno::EINVAL);
    }
    if len == 0 {
        return Ok(None);
    }
    let Some(end) = page_len(len).and_then(|l| addr.checked_add(l)).filter(|&e| e > addr) else {
        return Err(errno::ENOMEM);
    };
    if prot & !(PROT_ANY | PROT_SEM | PROT_GROWSDOWN | PROT_GROWSUP) != 0 {
        return Err(errno::EINVAL);
    }
    /*
     * A grow bit carries the change to the far end of a mapping that grows,
     * and no mapping here does: the stack is laid down whole. Linux refuses
     * the bit the same way on a mapping without VM_GROWSDOWN.
     */
    if grows != 0 {
        return Err(errno::EINVAL);
    }
    if wx_refused(prot) {
        return Err(errno::EPERM);
    }
    /* Nothing is mapped above the top of the guest's area. */
    if end > USER_MAX {
        return Err(errno::ENOMEM);
    }
    Ok(Some((addr, end - addr)))
}
