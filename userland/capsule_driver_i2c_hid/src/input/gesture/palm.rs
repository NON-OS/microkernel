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
use super::types::TouchGesture;

impl TouchGesture {
    /// Palm suppression: once a non-confident contact is seen, hold all
    /// motion until every contact has lifted. Typing with a palm resting on
    /// the pad must not steer the cursor. True while the frame is held.
    pub(super) fn palm_held(&mut self, tip: bool, confidence: bool, contacts: u32) -> bool {
        if tip && !confidence {
            self.palm = true;
        }
        if !self.palm {
            return false;
        }
        if !tip && contacts == 0 {
            self.palm = false;
        }
        true
    }
}
