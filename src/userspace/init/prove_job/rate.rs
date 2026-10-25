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

//! The DELIVER rate (plan/15): a supervisor measurement delivers at most
//! once per RATE_MS. Every capsule_linux supervisor shares one measurement,
//! so that is one Linux delivery per machine per span, as intended.

const RATE_MS: u64 = 10 * 60 * 1000;
/// Measurements remembered; the one longest unseen gives way.
const SEEN: usize = 8;

pub(super) struct Rate([([u8; 32], u64); SEEN]);

impl Rate {
    pub(super) const fn new() -> Rate {
        Rate([([0; 32], 0); SEEN])
    }

    /// Whether `m` may deliver at `now`; if so, it is noted as having done so.
    pub(super) fn take(&mut self, m: &[u8; 32], now: u64) -> bool {
        let at = self.0.iter().position(|s| s.0 == *m && s.1 != 0);
        if at.is_some_and(|i| now.saturating_sub(self.0[i].1) < RATE_MS) {
            return false;
        }
        let oldest = (0..SEEN).min_by_key(|&i| self.0[i].1).unwrap_or(0);
        self.0[at.unwrap_or(oldest)] = (*m, now.max(1));
        true
    }
}
