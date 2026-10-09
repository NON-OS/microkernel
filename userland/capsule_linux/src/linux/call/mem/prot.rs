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

//! `mprotect`, and the rule that makes it necessary. Its arguments are
//! checked in `prot_args`, which the host proofs hold.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub use super::prot_args::{PROT_ANY, PROT_EXEC, PROT_WRITE};

pub fn mprotect(guest: &mut Guest, addr: u64, len: u64, prot: u64) -> u64 {
    /*
     * Checked before anything else, because `len` is the guest's: `addr +
     * len` wraps, and a span computed from the wrapped value comes out
     * enormous.
     */
    let (start, span) = match super::prot_args::prot_span(addr, len, prot) {
        Ok(Some(span)) => span,
        Ok(None) => return errno::ok(0),
        Err(e) => return errno::fail(e),
    };
    /*
     * A file mapped without exec was never proved, and making it executable
     * now would run bytes the exec path would have refused. Anonymous memory
     * may still become executable, as a JIT needs; that is the guest's own
     * code, confined by its token rather than by provenance.
     */
    if prot & PROT_EXEC != 0 && guest.span_unproven(start, span) {
        return errno::fail(errno::EPERM);
    }
    super::prot_walk::walk(guest, start, span, prot)
}
