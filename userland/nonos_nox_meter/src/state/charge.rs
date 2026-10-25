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
use crate::channel::Channel;
use crate::error::MeterError;

impl MeterState {
    /// The state after charging `amount`, with usage root `root`, to the live
    /// channel for this payee, listing and chain, and the channel as charged.
    /// Refused unless both the channel's deposit and the owner's budget cover
    /// it. Of the live channels, one with room for the charge is preferred, then
    /// the one that lasts longer.
    pub fn charged(
        &self,
        payee: &[u8; 20],
        listing: &[u8; 32],
        chain: u64,
        amount: u128,
        root: [u8; 32],
        now: u64,
    ) -> Result<(MeterState, Channel), MeterError> {
        let at = self.live(payee, listing, chain, amount, now).ok_or(MeterError::NoChannel)?;
        let channel = self.channels[at].charged(amount, root, now)?;
        let budget = self.budget.spending(listing, amount, now)?;
        let mut next = MeterState { budget, channels: self.channels.clone(), ..self.clone() };
        next.channels[at] = channel;
        Ok((next, channel))
    }

    fn live(
        &self,
        payee: &[u8; 20],
        listing: &[u8; 32],
        chain: u64,
        amount: u128,
        now: u64,
    ) -> Option<usize> {
        (0..self.channels.len())
            .filter(|&i| {
                let c = &self.channels[i];
                &c.payee == payee && &c.listing_id == listing && c.chain_id == chain && c.live(now)
            })
            .max_by_key(|&i| {
                let c = &self.channels[i];
                let room = c.deposit.saturating_sub(c.cumulative.max(c.settled));
                (room >= amount && !c.frozen, c.expiry)
            })
    }
}
