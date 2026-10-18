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

use super::state::Channel;

impl Channel {
    /// This channel as settle reports it (plan/21, v1.3 E13). The settled
    /// amount raises the floor only up to the largest cumulative this meter
    /// signed. A report above that is either a lying RPC or a voucher this
    /// meter never made, so the channel freezes and takes no further charge,
    /// and its numbers stay as they were. The deposit is the chain's word.
    pub fn synced(&self, deposit: u128, settled: u128) -> Channel {
        if settled > self.cumulative {
            return Channel { deposit, frozen: true, ..*self };
        }
        Channel { deposit, settled: self.settled.max(settled), ..*self }
    }
}
