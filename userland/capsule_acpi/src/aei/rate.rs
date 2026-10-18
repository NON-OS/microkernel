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

//! How many events one GPIO controller may signal a second. A pin stuck
//! asserted, or a driver reporting in a loop, would otherwise run AML
//! without end; past the bound an event is refused (EAGAIN) and counted,
//! and the next second starts afresh. Pure: `capsule_acpi_proofs` mounts it.

pub const PER_SECOND: u32 = 64;
const SECOND_MS: u64 = 1000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rate {
    pub window_ms: u64,
    pub count: u32,
    pub refused: u64,
}

impl Rate {
    pub fn admit(&mut self, now_ms: u64) -> bool {
        if now_ms.saturating_sub(self.window_ms) >= SECOND_MS || now_ms < self.window_ms {
            (self.window_ms, self.count) = (now_ms, 0);
        }
        if self.count >= PER_SECOND {
            self.refused += 1;
            return false;
        }
        self.count += 1;
        true
    }
}
