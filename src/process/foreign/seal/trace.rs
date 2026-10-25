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

//! K, a family's trace chain: every call its guests made, in the order the
//! supervisor's answers reached them. The kernel records numbers here and
//! learns nothing of what they mean.

/// What a row says happened to the call.
pub(crate) const VALUE: u32 = 0;
pub(crate) const EXECED: u32 = 1;
pub(crate) const DELIVER: u32 = 2;
pub(crate) const SIGRETURN: u32 = 3;
pub(crate) const ABANDONED: u32 = 4;
pub(crate) const DIED: u32 = 5;
pub(crate) const REFUSED: u32 = 6;

#[derive(Clone, Copy)]
pub(crate) struct Row {
    /// Stamped when the call trapped, so answers out of order still show.
    pub(crate) seq: u64,
    pub(crate) pid: u32,
    pub(crate) what: u32,
    pub(crate) nr: u64,
    pub(crate) ret: u64,
}

/// K = BLAKE3(K | seq | pid | what | nr | ret), all little-endian.
pub(crate) fn trace_step(k: &[u8; 32], row: &Row) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(k);
    h.update(&row.seq.to_le_bytes());
    h.update(&row.pid.to_le_bytes());
    h.update(&row.what.to_le_bytes());
    h.update(&row.nr.to_le_bytes());
    h.update(&row.ret.to_le_bytes());
    *h.finalize().as_bytes()
}
