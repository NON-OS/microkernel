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

//! One zone's readings over time (ACPI 6.5, 11.1, and the rulings on _CRT).
//! _CRT is believed only on two readings at least `CONFIRM_MS` apart, since
//! one stray _TMP must not power the machine off; a zone at or past _CRT is
//! read every `CONFIRM_MS` until that is settled, and every
//! `CRITICAL_POLL_MS` while it lasts. A reading cut short, absent or not
//! believed (F-127) is no reading: the zone never reads as cooler for it.
//! Its last temperature and verdict stand, and the critical watch neither
//! starts, settles nor ends on it. Our bounds, from no source.

use super::trips::Trips;
use super::verdict::{judge, Verdict};

pub const CONFIRM_MS: u64 = 1_000;
pub const CRITICAL_POLL_MS: u64 = 5_000;
/// A told bit beside the trip faults: the zone's _TMP was unread.
pub const UNREAD_TMP: u16 = 1 << 7;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Watch {
    /// In m°C, the last temperature believed; None until one is.
    pub temp: Option<i32>,
    /// The latest reading failed; `temp` and `verdict` are from before it.
    pub unread: bool,
    pub verdict: Verdict,
    /// When _CRT was first reached, waiting on a second reading.
    pub critical_at: Option<u64>,
    /// _CRT reached on two readings `CONFIRM_MS` apart.
    pub critical: bool,
}

impl Watch {
    /// One reading at `now`; the wait until the next, `poll` when nothing
    /// about _CRT is pending.
    pub fn reading(&mut self, t: &Trips, read: Option<i32>, now: u64, poll: u64) -> u64 {
        self.unread = read.is_none();
        if let Some(temp) = read {
            let v = judge(t, temp, &self.verdict.active);
            self.critical = match (v.critical, self.critical_at) {
                (false, _) => {
                    self.critical_at = None;
                    false
                }
                (true, None) => {
                    self.critical_at = Some(now);
                    false
                }
                (true, Some(at)) => self.critical || now.saturating_sub(at) >= CONFIRM_MS,
            };
            (self.temp, self.verdict) = (Some(temp), v);
        }
        match (self.critical_at.is_some(), self.critical) {
            (true, false) => CONFIRM_MS,
            (true, true) => CRITICAL_POLL_MS,
            _ => poll,
        }
    }
}
