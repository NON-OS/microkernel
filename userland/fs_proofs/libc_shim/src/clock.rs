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

//! The loader's clock. Each reading is one millisecond past the last, so a
//! time-budgeted slice ends after a handful of chunks and a proof walks the
//! resumable path rather than finishing every load in one call.

use std::cell::Cell;

thread_local! {
    static NOW: Cell<i64> = const { Cell::new(0) };
}

pub fn mk_uptime_ms() -> i64 {
    NOW.with(|now| {
        let t = now.get() + 1;
        now.set(t);
        t
    })
}
