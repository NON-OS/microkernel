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

//! A supervisor family's seal: closing it, noting in it, and reading it
//! back once sealed.

use crate::syscall::{call_raw, N_MK_FOREIGN_SEAL, N_MK_SEAL_NOTE, N_MK_SEAL_READ};

/// The record, 240 bytes, public.
pub const SEAL_RECORD: u64 = 0;
/// The histogram, its salt and the diag, for nonos.prove alone.
pub const SEAL_WITNESS: u64 = 1;
/// The measurements, 32 bytes each, in the order the kernel chained them.
pub const SEAL_MEASUREMENTS: u64 = 2;

/// The app a family runs, noted once before its first call.
pub const SEAL_APP_ID: u64 = 0;
/// The supervisor's own diagnostic head; the last note is kept.
pub const SEAL_DIAG: u64 = 1;

/// Seal the caller's family once no guest is left: the seal number, or a
/// negative errno (EBUSY while a guest lives, ENOENT with nothing to seal).
pub fn mk_foreign_seal() -> i64 {
    call_raw(N_MK_FOREIGN_SEAL, [0; 6])
}

/// One part of a sealed family into `out`: bytes written, or a negative errno.
pub fn mk_seal_read(seal_no: u64, part: u64, out: &mut [u8]) -> i64 {
    call_raw(N_MK_SEAL_READ, [seal_no, part, out.as_mut_ptr() as u64, out.len() as u64, 0, 0])
}

/// Note `word` in the caller's live family under `slot`.
pub fn mk_seal_note(slot: u64, word: &[u8; 32]) -> i64 {
    call_raw(N_MK_SEAL_NOTE, [slot, word.as_ptr() as u64, 0, 0, 0, 0])
}
