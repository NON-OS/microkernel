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

//! The pad's interrupt line, sensed by the controller (OP_GPIO_DOORBELL).
//! The pad holds it asserted while a report waits, so a quiet line means no
//! read at all: no stale re-reads, no torn frames. It must ring once before
//! it is trusted (timed reads go on until then, so a line the firmware
//! never wired cannot silence input), and its trust lapses after TRUST_MS
//! of quiet, so a one-off spurious reading cannot lock the reads out.

use crate::i2c_client::query_doorbell;
use crate::state::State;

const TRUST_MS: u64 = 5_000;
/// Unanswered asks after which the controller is taken to have no line.
const GIVE_UP: u32 = 3;

#[derive(Clone, Copy, Default)]
pub struct Doorbell {
    /// It rang once, proving the line tracks this pad.
    pub proven: bool,
    /// The controller has none, or it failed: reads go by the timer and it
    /// is not asked again.
    pub absent: bool,
    failures: u32,
    /// Rings in a row whose read found no report (rang.rs).
    pub(super) misses: u32,
    rang_ms: u64,
}

pub(super) enum Look {
    /// No line to go by: read on the timer.
    Timed,
    Rang,
    /// A trusted line, quiet: nothing to read.
    Quiet,
}

pub(super) fn look(state: &mut State, now_ms: u64) -> Look {
    let db = &mut state.doorbell;
    if db.absent {
        return Look::Timed;
    }
    let Some((present, fired)) = query_doorbell(state.i2c_port) else {
        db.failures = db.failures.saturating_add(1);
        db.absent = db.failures >= GIVE_UP;
        return Look::Timed;
    };
    db.failures = 0;
    db.absent = !present;
    if fired && present {
        (db.proven, db.rang_ms) = (true, now_ms);
        return Look::Rang;
    }
    if db.proven && now_ms.saturating_sub(db.rang_ms) > TRUST_MS {
        db.proven = false;
    }
    match db.proven && present {
        true => Look::Quiet,
        false => Look::Timed,
    }
}
