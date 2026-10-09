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

//! A clock two guests could read to agree on a moment.
//!
//! Monotonic time since boot is the machine's, the same for every guest on it
//! and different on the next machine. A family's clocks start with the family,
//! so a guest reading one at its own start must see a small number.

use crate::report::{Report, Seen};
use crate::sys::call;

const CLOCK_GETTIME: u64 = 228;
const CLOCKS: [(u64, &str); 2] = [(1, "monotonic"), (7, "boottime")];
/// Far above a family's first moments, far below any boot under emulation.
const FRESH_MS: u64 = 30_000;

/// Called first thing in a guest's main, before anything else takes time.
pub fn scan(r: &mut Report) {
    for (clock, name) in CLOCKS {
        let mut ts = [0u64; 2];
        let rc = call(CLOCK_GETTIME, [clock, ts.as_mut_ptr() as u64, 0, 0, 0, 0]);
        let ms = ts[0].saturating_mul(1000).saturating_add(ts[1] / 1_000_000);
        let seen = match (rc, ms) {
            (rc, _) if rc < 0 => Seen::Refused(rc),
            (_, ms) if ms < FRESH_MS => Seen::Refused(0),
            (_, ms) => Seen::Escaped(format!("{ms} ms, the machine's uptime")),
        };
        r.check(&format!("the {name} clock"), seen);
    }
}
