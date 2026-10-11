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
use crate::channel::{Channel, MAX_CHANNELS};
use crate::error::MeterError;

impl MeterState {
    /// The state knowing `channel`, opened on chain. A channel already known
    /// only takes the chain's deposit and settled amount: opening again never
    /// rewinds what was signed. A full table makes room by forgetting one
    /// channel that is past expiry and fully settled, never any other.
    pub fn with_channel(&self, channel: Channel, now: u64) -> Result<MeterState, MeterError> {
        if self.channels.iter().any(|c| c.id == channel.id) {
            return Ok(self.synced(&channel.id, channel.deposit, channel.settled));
        }
        let mut next = self.clone();
        if next.channels.len() >= MAX_CHANNELS {
            let spent = next
                .channels
                .iter()
                .position(|c| c.expired(now) && c.settled >= c.cumulative)
                .ok_or(MeterError::Full)?;
            next.channels.remove(spent);
        }
        next.channels.push(channel);
        Ok(next)
    }

    /// The state with `sig` recorded for the voucher `seq` on channel `id`.
    /// A signature for any voucher but the newest is not recorded: it would
    /// hand DRAIN a voucher the state no longer describes.
    pub fn signed(&self, id: &[u8; 32], seq: u64, sig: [u8; 65]) -> MeterState {
        let mut next = self.clone();
        if let Some(c) = next.channels.iter_mut().find(|c| &c.id == id && c.seq == seq) {
            c.sig = sig;
        }
        next
    }
}
