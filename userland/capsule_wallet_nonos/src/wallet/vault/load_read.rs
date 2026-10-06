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

//! Reading the sealed blob back off an open descriptor.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::read_judge::judge;
pub(super) use super::read_judge::Unread;
use super::vfs::{call, HDR_LEN, OP_READ};

/// The blob, or why there is none (`read_judge`).
pub(super) fn read_exact<const N: usize>(fd: u32) -> Result<[u8; N], Unread> {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(12);
    body.extend_from_slice(&pid.to_le_bytes());
    body.extend_from_slice(&fd.to_le_bytes());
    body.extend_from_slice(&(N as u32).to_le_bytes());

    let mut rx = vec![0u8; HDR_LEN + 8 + N];
    judge::<N>(call(OP_READ, &body, &mut rx), &rx, HDR_LEN + 4)
}
