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

//! Reading an IPC reply. The kernel reports how many bytes it wrote; that
//! count, and every length inside the reply, is judged against the buffer
//! before a byte past the header is read.

/// The reply the kernel wrote into `buf`: its first `n` bytes, or None if
/// `n` is an error or more than `buf` holds.
pub fn written(buf: &[u8], n: i64) -> Option<&[u8]> {
    let n = usize::try_from(n).ok()?;
    buf.get(..n)
}

/// The payload of `plen` bytes after a header of `hdr` bytes, if `reply`
/// holds all of it.
pub fn payload(reply: &[u8], hdr: usize, plen: usize) -> Option<&[u8]> {
    reply.get(hdr..hdr.checked_add(plen)?)
}
