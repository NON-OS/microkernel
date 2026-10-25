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

//! The frame's 64-bit count, read as two 32-bit halves.

use super::frame::{Frame, CNTPCT_HI, CNTPCT_LO};
use super::halves::combine;

impl Frame {
    /// The system count by halves: high, low, high (`combine`).
    pub(super) fn count(&self) -> u64 {
        let high = self.read(CNTPCT_HI);
        let low = self.read(CNTPCT_LO);
        combine(high, low, self.read(CNTPCT_HI))
    }
}
