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

//! Taking in what one slice brought, against the flight's bounds.

use super::{Gather, Gathered};

impl Gather {
    /// What one slice brought, and whether the far end said it finished,
    /// at `now`. `done` says the flight is whole.
    pub fn take(
        &mut self,
        bytes: &[u8],
        ended: bool,
        now: i64,
        mut done: impl FnMut(&[u8]) -> bool,
    ) -> Gathered {
        if !bytes.is_empty() {
            self.out.extend_from_slice(bytes);
            self.last = now;
            if self.out.len() > self.bounds.max {
                self.out.clear();
                return Gathered::Nothing;
            }
            if done(&self.out) {
                return self.finish();
            }
        }
        if ended {
            return self.finish();
        }
        let wait = if self.out.is_empty() { self.bounds.first_ms } else { self.bounds.quiet_ms };
        let quiet = now.saturating_sub(self.last) >= clamp(wait);
        let spent = now.saturating_sub(self.start) >= clamp(self.bounds.total_ms);
        if quiet || spent {
            return self.finish();
        }
        Gathered::More
    }
}

fn clamp(ms: u64) -> i64 {
    i64::try_from(ms).unwrap_or(i64::MAX)
}
