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
 * Handing the kernel a mirror's bytes, and why pouring them stopped short.
 */

use alloc::vec::Vec;

use crate::feed::feed;
use crate::http::Fault;

/* The most one call to the kernel takes. */
pub(super) const BATCH: usize = 1 << 20;

/* Why pouring stopped short of the whole file. */
pub enum Stop {
    Mirror(Fault),
    /* The kernel refused a batch; its errno. */
    Volume(i64),
}

pub(super) fn broke() -> Stop {
    Stop::Mirror(Fault::Net("the connection to the mirror broke"))
}

/* Feed the batch, never more than `BATCH` bytes a call; `at` is where the kernel says it is. */
pub(super) fn put(batch: &mut Vec<u8>, at: &mut u64) -> Result<(), Stop> {
    for part in batch.chunks(BATCH) {
        *at = feed(part).map_err(Stop::Volume)?;
    }
    batch.clear();
    Ok(())
}
