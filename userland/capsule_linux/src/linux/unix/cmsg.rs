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

//! The descriptors an SCM_RIGHTS control block carries, walked as Linux's
//! __cmsg_nxthdr walks it: each header's length is checked against what is
//! left of the block before it is used, so no length a guest writes can
//! step back, stand still or reach past the end. Pure, so the host proofs
//! hold it against any bytes at all.

use alloc::vec::Vec;

/// struct cmsghdr: cmsg_len, cmsg_level, cmsg_type, then the data.
const CMSG_HDR: usize = 16;
const SOL_SOCKET: u32 = 1;
const SCM_RIGHTS: u32 = 1;
/// SCM_MAX_FD: the most descriptors one message passes.
pub const MAX_FDS: usize = 253;

/// Every descriptor in the SCM_RIGHTS blocks of `raw`, at most MAX_FDS.
/// Anything else in the control data is stepped over, never guessed at; a
/// header whose length is shorter than a header or longer than what is
/// left ends the walk.
pub fn rights(raw: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while let Some(rest) = raw.get(off..).filter(|r| r.len() >= CMSG_HDR) {
        let word =
            |at: usize| u32::from_le_bytes([rest[at], rest[at + 1], rest[at + 2], rest[at + 3]]);
        let len = u64::from(word(0)) | u64::from(word(4)) << 32;
        let Some(len) = usize::try_from(len).ok().filter(|&l| (CMSG_HDR..=rest.len()).contains(&l))
        else {
            break;
        };
        if word(8) == SOL_SOCKET && word(12) == SCM_RIGHTS {
            for fd in rest[CMSG_HDR..len].chunks_exact(4) {
                if out.len() == MAX_FDS {
                    return out;
                }
                out.push(u32::from_le_bytes([fd[0], fd[1], fd[2], fd[3]]));
            }
        }
        /* CMSG_ALIGN: the next header starts on an eight-byte boundary. */
        off += (len + 7) & !7;
    }
    out
}
