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
use crate::constants::STATUS_OUTPUT_FULL;
use crate::constants::{DATA_OFFSET, STATUS_OFFSET};
use nonos_libc::mk_pio_read;
const MAX_FLUSH_BYTES: u32 = 16;

/// Drain stale bytes from the output buffer, at most `MAX_FLUSH_BYTES`.
///
/// Answers false only when the buffer still shows full after the bound: a
/// port with no i8042 behind it floats and reads 0xFF, which looks like a
/// full buffer forever, and that is how a machine without a controller looks
/// (Linux's i8042_controller_check makes the same call). A refused read
/// proves nothing either way and answers true.
pub fn flush_output(grant_id: u64) -> bool {
    for _ in 0..MAX_FLUSH_BYTES {
        let mut status: u32 = 0;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return true;
        }
        if (status as u8) & STATUS_OUTPUT_FULL == 0 {
            return true;
        }
        let mut sink: u32 = 0;
        let _ = mk_pio_read(grant_id, DATA_OFFSET, 1, &mut sink);
    }
    false
}
