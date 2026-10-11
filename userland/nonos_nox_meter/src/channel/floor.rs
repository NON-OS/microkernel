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
    /// This channel as the chain reports it. What the contract settled is a
    /// floor under `cumulative`: a state file rolled back to an older copy
    /// cannot make the meter forget spend the chain has already paid. The
    /// deposit is the chain's word outright. Returns how far the floor rose.
    pub fn synced(&self, deposit: u128, settled: u128) -> (Channel, u128) {
        let settled = self.settled.max(settled);
        let cumulative = self.cumulative.max(settled);
        let rose = cumulative - self.cumulative;
        (Channel { deposit, settled, cumulative, ..*self }, rose)
    }
}
