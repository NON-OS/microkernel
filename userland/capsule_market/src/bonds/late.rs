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

//! A refresh's end: its row from the three answers, or its drop when late.
//! Pure.

use nonos_marketplace_abi::BondReading;

use super::cache::{BondCache, Row};
use super::refresh::{Ask, Step, STEP_MS};

impl BondCache {
    /// Drop a step unanswered for STEP_MS; the next ask, if one was dropped.
    pub fn tick(&mut self, now_ms: u64) -> Option<Ask> {
        let late = self.step.as_ref().is_some_and(|s| now_ms.saturating_sub(s.sent_ms) >= STEP_MS);
        late.then(|| self.step = None).and_then(|_| self.next(now_ms))
    }

    pub fn waiting(&self) -> bool {
        self.step.is_some()
    }
}

pub(super) fn row_of(s: &Step, now: u64) -> Option<Row> {
    let [first, second, third] = s.got.as_slice() else { return None };
    let reading = BondReading::from_words(first, second, now)?;
    let [m] = third.as_slice() else { return None };
    let (high, low) = m.split_at(16);
    high.iter().all(|b| *b == 0).then_some(())?;
    let min = u128::from_be_bytes(low.try_into().ok()?);
    Some(Row { key: s.key, reading, min })
}
