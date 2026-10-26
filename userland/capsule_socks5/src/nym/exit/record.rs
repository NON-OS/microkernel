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

//! What the current exit has done: its sends, answers and deliveries, and
//! which exits went silent or delivered this session.

use spin::Mutex;

use super::super::choose::Record;
use super::super::watch::Watch;
use super::chosen::{Exit, EXIT};

/// Delivery record for the exit in `EXIT`. See `watch` for the rule it
/// enforces: lookups prove nothing, only delivery does.
pub(super) static WATCH: Mutex<Watch> = Mutex::new(Watch::new());

/// Which exits went silent on this session and which delivered (`choose`).
pub(super) static RECORD: Mutex<Record<Exit>> = Mutex::new(Record::new());

/// A send left for the current exit.
pub fn note_sent() {
    WATCH.lock().on_send(nonos_libc::mk_uptime_ms());
}

/// A message came back through the current exit, which proves it.
pub fn note_delivered() {
    WATCH.lock().on_delivered();
    if let Some(exit) = *EXIT.lock() {
        RECORD.lock().proven(exit);
    }
}

/// The current exit answered without payload: alive, not yet proven.
pub fn note_answered() {
    WATCH.lock().on_answered(nonos_libc::mk_uptime_ms());
}
