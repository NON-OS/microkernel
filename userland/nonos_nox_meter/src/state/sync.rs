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

use super::meter::MeterState;

impl MeterState {
    /// The state after the chain reported `deposit` and `settled` for one
    /// channel. Spend the chain paid that this state had not counted, as a
    /// rolled-back file would hide, is counted against the budget too.
    pub fn synced(&self, channel_id: &[u8; 32], deposit: u128, settled: u128) -> MeterState {
        let mut next = self.clone();
        if let Some(c) = next.channels.iter_mut().find(|c| &c.id == channel_id) {
            let (channel, rose) = c.synced(deposit, settled);
            *c = channel;
            next.budget.absorb(&channel.listing_id, rose);
        }
        next
    }
}
