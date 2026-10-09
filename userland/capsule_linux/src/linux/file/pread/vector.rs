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

/* preadv2 and pwritev2 with their RWF_ flags. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::place::place;
use super::plain::at_offset;
use super::sync::synced;

/* The RWF_ flags Linux 6.1 knows: HIPRI, DSYNC, SYNC, NOWAIT and APPEND. */
const RWF_DSYNC: u64 = 0x02;

const RWF_SYNC: u64 = 0x04;

const RWF_APPEND: u64 = 0x10;

const RWF_KNOWN: u64 = 0x1f;

/*
 * The v2 forms: an offset of -1 means the descriptor's own, which then
 * moves. HIPRI asks to poll for completion and NOWAIT not to block, and a
 * read or write here never blocks, so both hold as they are. DSYNC and
 * SYNC put a write in the store before answering, as fsync would. APPEND
 * writes at the end whatever the offset. Any other flag is EOPNOTSUPP.
 */
pub(super) fn vectored(
    guest: &mut Guest,
    fd: u64,
    at: u64,
    flags: u64,
    write: bool,
    go: impl FnOnce(&mut Guest) -> u64,
) -> u64 {
    if flags & !RWF_KNOWN != 0 {
        return errno::fail(errno::EOPNOTSUPP);
    }
    let at = match place(guest, fd, at, write && flags & RWF_APPEND != 0) {
        Ok(at) => at,
        Err(e) => return e,
    };
    let got = match at {
        u64::MAX => go(guest),
        _ => at_offset(guest, fd, at, go),
    };
    if write && flags & (RWF_DSYNC | RWF_SYNC) != 0 && (got as i64) >= 0 {
        if let Err(e) = synced(guest, fd) {
            return errno::fail(e);
        }
    }
    got
}
