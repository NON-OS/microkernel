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

//! The agent's rate limit per listing (plan/29): it serves one release, so one
//! bucket of `per_minute` answers, refilled evenly, covers the tool.

const TOKEN: u64 = 60_000;

pub struct Bucket {
    per_minute: u32,
    /// Tokens in 1/60000ths: a minute's refill of per_minute tokens is
    /// exactly per_minute units a millisecond, so no fraction is lost.
    units: u64,
    last_ms: u64,
}

impl Bucket {
    pub fn new(per_minute: u32, now_ms: u64) -> Self {
        Bucket { per_minute, units: u64::from(per_minute) * TOKEN, last_ms: now_ms }
    }

    /// Takes one answer's token, or false when the minute's are spent.
    pub fn take(&mut self, now_ms: u64) -> bool {
        let cap = u64::from(self.per_minute) * TOKEN;
        let gained = now_ms.saturating_sub(self.last_ms).saturating_mul(u64::from(self.per_minute));
        self.units = self.units.saturating_add(gained).min(cap);
        self.last_ms = now_ms.max(self.last_ms);
        if self.units < TOKEN {
            return false;
        }
        self.units -= TOKEN;
        true
    }
}
