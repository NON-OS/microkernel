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
 * Reading a model straight into a guest: the kernel opens the model's
 * sealed sectors into the guest's own pages, so no copy of its bytes
 * passes through this capsule's memory on the way.
 */

use nonos_libc::mk_data_read_peer;

use crate::linux::guest::Guest;

use super::name::{owns, volume_name};

/*
 * The most one read moves. The kernel takes up to 4 MiB, but it opens the
 * range with interrupts masked, so a read is kept to what the rest of the
 * family's reads move.
 */
const MOST: u64 = 1 << 20;

/*
 * Up to `len` bytes of the model at `path` from `at`, put at `buf` in the
 * guest: the count, 0 at the end, or an errno. None for any path that is
 * not a model file on the volume; the catalog, made here, goes the usual
 * way, as does /models itself.
 */
pub fn read_into(
    guest: &Guest,
    path: &[u8],
    at: u64,
    buf: u64,
    len: u64,
) -> Option<Result<u64, i64>> {
    if !owns(path) || path == super::catalog::TIERS {
        return None;
    }
    let name = volume_name(path)?;
    if len == 0 {
        return Some(Ok(0));
    }
    let got = mk_data_read_peer(name, at, guest.pid, buf, len.min(MOST));
    Some(if got < 0 { Err(-got) } else { Ok(got as u64) })
}
