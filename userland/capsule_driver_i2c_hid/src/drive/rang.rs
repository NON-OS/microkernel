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

//! A read the interrupt line asked for, and what it found. The zero-length
//! report there is the one a device serves once it has reset itself (HID
//! over I2C 1.0, 7.2.1), so the pad is set up again (reset.rs). A line
//! that keeps ringing with no report is the wrong pin or the wrong
//! polarity, and trusting it would hold reads back exactly while the
//! finger moves: after MISS_LIMIT in a row it is given up.

use super::reset::reset_seen;
use crate::state::{LastRead, State};

const MISS_LIMIT: u32 = 16;

pub(super) fn after_ring(state: &mut State) {
    if state.last_read == LastRead::Report {
        state.doorbell.misses = 0;
        return;
    }
    if state.last_read == LastRead::Empty {
        reset_seen(state);
    }
    let db = &mut state.doorbell;
    db.misses = db.misses.saturating_add(1);
    if db.misses >= MISS_LIMIT {
        (db.absent, db.proven) = (true, false);
        crate::diag::line(alloc::format!(
            "[i2chid] doorbell rang {MISS_LIMIT} times with no report; reads go back to the timer\n"
        ));
    }
}
