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

//! A setup step that may meet a busy peer, tried a few times with a short
//! rest between. The compositor answers only between frames, and a full
//! frame with its transfer to the display can outlast one call's budget; a
//! single late answer used to send the whole desktop setup round again.

use nonos_libc::mk_idle_ms;

/// Run `step` up to `tries` times, resting `gap_ms` between; the last
/// error if none succeeds.
pub fn patiently<T>(
    tries: u32,
    gap_ms: u64,
    mut step: impl FnMut() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    let mut last = "no try was made";
    for attempt in 0..tries {
        match step() {
            Ok(v) => return Ok(v),
            Err(e) => last = e,
        }
        if attempt + 1 < tries {
            mk_idle_ms(gap_ms);
        }
    }
    Err(last)
}
