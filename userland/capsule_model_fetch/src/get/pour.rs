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
 * Pouring a mirror's bytes into the kernel's stream, a batch of up to 1 MiB
 * at a time, the most one call to the kernel takes.
 */

use alloc::vec::Vec;

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::progress::Progress;
use super::put::{broke, put, Stop, BATCH};
use crate::http::Body;

/* No byte for this long, skipped or kept, and the connection is taken as gone. */
const QUIET_MS: i64 = 60_000;

pub fn pour(body: &mut Body, at: &mut u64, bytes: u64, show: &mut Progress) -> Result<(), Stop> {
    let mut batch = Vec::with_capacity(BATCH);
    let mut quiet_until = mk_uptime_ms().saturating_add(QUIET_MS);
    while *at < bytes {
        let skip = body.skip;
        let Ok(got) = body.next() else { return put(&mut batch, at).and(Err(broke())) };
        /* Bytes a mirror that ignored the range sent again are bytes all the same. */
        let moved = !got.is_empty() || body.skip < skip;
        if moved {
            quiet_until = mk_uptime_ms().saturating_add(QUIET_MS);
        }
        if got.is_empty() {
            if body.ended() || mk_uptime_ms() > quiet_until {
                return put(&mut batch, at).and(Err(broke()));
            }
            if !moved {
                mk_idle_ms(2);
            }
            continue;
        }
        let room = (bytes - *at) as usize - batch.len();
        batch.extend_from_slice(&got[..got.len().min(room)]);
        if batch.len() >= BATCH || batch.len() as u64 == bytes - *at {
            put(&mut batch, at)?;
            show.at(*at);
        }
    }
    Ok(())
}
