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

//! How often the serve loop brings a mouse that reset up again. Each time
//! holds the keyboard port off for the configuration exchange, and a mouse
//! that resets for ever would have the driver doing nothing else.

/// Bring-ups allowed within WINDOW_MS; the next reset in it turns the
/// pointer off for the session.
const REINITS: usize = 3;
const WINDOW_MS: u64 = 60_000;

/// When the last REINITS bring-ups ran, oldest first.
#[derive(Default)]
pub struct Reinits([Option<u64>; REINITS]);

impl Reinits {
    /// Count a bring-up at `now_ms`, or refuse it: false when REINITS ran
    /// within the WINDOW_MS before it.
    pub fn allow(&mut self, now_ms: u64) -> bool {
        if self.0[0].is_some_and(|oldest| now_ms.saturating_sub(oldest) < WINDOW_MS) {
            return false;
        }
        self.0.rotate_left(1);
        self.0[REINITS - 1] = Some(now_ms);
        true
    }
}
