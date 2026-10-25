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

//! The bad states a move reached.

use super::world::{pid, World, P, WHOLE};

impl World {
    /// Marks the bad states the move reached; always true.
    pub(super) fn judge(&mut self) -> bool {
        let inside: Vec<usize> = (0..4).filter(|&i| self.live_inside(i)).collect();
        self.bad |= 0x01 * (inside.len() > 1 && inside.iter().any(|&i| WHOLE[i])) as u16;
        for i in 0..4 {
            let lost = self.p[i] == P::Waiting
                && self.alive[i]
                && !self.woken[i]
                && !self.rows.waiting.contains(&pid(i));
            self.bad |= 0x04 * lost as u16;
        }
        true
    }
}
