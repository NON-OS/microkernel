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
use super::state::{Channel, NO_SIG};
use crate::error::MeterError;

impl Channel {
    /// This channel after a charge of `amount` whose usage root is `root`, or
    /// why there is none. Pure: the caller writes the result to disk before
    /// any voucher for it leaves, so a crash can lose a charge but never sign
    /// below one already signed.
    pub fn charged(&self, amount: u128, root: [u8; 32], now: u64) -> Result<Channel, MeterError> {
        if now >= self.expiry {
            return Err(MeterError::Expired);
        }
        let base = self.cumulative.max(self.settled);
        let cumulative = base.checked_add(amount).ok_or(MeterError::Overflow)?;
        if cumulative > self.deposit {
            return Err(MeterError::OverDeposit);
        }
        let seq = self.seq.checked_add(1).ok_or(MeterError::Overflow)?;
        Ok(Channel { cumulative, seq, usage_root: root, sig: NO_SIG, ..*self })
    }

    /// Whether a charge may land here at `now`.
    pub fn live(&self, now: u64) -> bool {
        now < self.expiry
    }
}
