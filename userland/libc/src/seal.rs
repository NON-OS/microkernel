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

//! A supervisor family's seal: noting in it while it lives, and taking it
//! once the kernel has sealed it, when its last guest left.

use crate::syscall::{call_raw, N_MK_SEAL_NOTE, N_MK_SEAL_READ};

/// The app a family runs, noted once before its first guest starts.
pub const SEAL_APP_ID: u64 = 0;
/// The supervisor's own diagnostic head; the last note is kept.
pub const SEAL_DIAG: u64 = 1;

/// Take the newest final record sealed for `app_id` into `out`, as record
/// (240 bytes) | witness | measurement list, 32 bytes each. Bytes written,
/// or a negative errno: ENOENT with no such record, ENOMEM when `out` is
/// short. nonos.prove's alone; the record is dropped once read.
pub fn mk_seal_read(app_id: &[u8; 32], out: &mut [u8]) -> i64 {
    let args = [app_id.as_ptr() as u64, out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0];
    call_raw(N_MK_SEAL_READ, args)
}

/// Note `word` in the caller's live family under `slot`.
/// The live form: the family `pid` supervises, as it stands, for
/// DELIVER; nothing is dropped. ESRCH when it supervises none.
pub fn mk_seal_read_live(pid: u32, out: &mut [u8]) -> i64 {
    let args = [0, out.as_mut_ptr() as u64, out.len() as u64, u64::from(pid), 0, 0];
    call_raw(N_MK_SEAL_READ, args)
}

pub fn mk_seal_note(slot: u64, word: &[u8; 32]) -> i64 {
    call_raw(N_MK_SEAL_NOTE, [slot, word.as_ptr() as u64, 0, 0, 0, 0])
}
