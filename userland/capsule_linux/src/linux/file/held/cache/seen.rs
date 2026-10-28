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

/*
 * A copy written, cut or grown, as every shared map of the file in the
 * family sees it (shared_maps). Every write of a file's bytes ends here:
 * write, the p- and v- forms, sendfile, copy_file_range and fallocate's
 * zeroes; and every change of its length: ftruncate, truncate and an
 * open with O_TRUNC.
 */

use super::super::shared_maps;
use super::change::{put, set_len};
use super::table::size;

/*
 * Write `bytes` at `at`, and show them to every map of them. A write past
 * the end leaves a gap that reads zero, in the maps as in the file.
 */
pub fn write(path: &[u8], at: u64, bytes: &[u8]) -> Result<usize, i64> {
    let was = size(path).unwrap_or(0);
    let n = put(path, at, bytes)?;
    if at > was {
        shared_maps::truncated(path, was, at);
    }
    shared_maps::wrote(path, at, &bytes[..n]);
    Ok(n)
}

/* Cut or grow the copy to `len`: its maps read zero between the ends. */
pub fn resize(path: &[u8], len: u64) -> Result<(), i64> {
    let was = size(path).unwrap_or(0);
    set_len(path, len)?;
    shared_maps::truncated(path, was, len);
    Ok(())
}

/* A copy just taken empty from a stored file of `stored` bytes. */
pub fn emptied(path: &[u8], stored: u64) -> Result<(), i64> {
    shared_maps::truncated(path, stored, 0);
    Ok(())
}
