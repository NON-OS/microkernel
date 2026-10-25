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

//! INTENT's reply body: kind u8 | to[20] | nonce u64 | gasLimit u64 |
//! maxFee u128 | maxPriority u128. gasLimit is the estimate times 6/5; the
//! fees are suggestions the signer bounds.

pub const QUOTE_LEN: usize = 1 + 20 + 8 + 8 + 16 + 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Quote {
    pub kind: u8,
    pub to: [u8; 20],
    pub nonce: u64,
    pub gas_limit: u64,
    pub max_fee: u128,
    pub max_priority: u128,
}

impl Quote {
    pub fn encode(&self) -> [u8; QUOTE_LEN] {
        let mut out = [0u8; QUOTE_LEN];
        out[0] = self.kind;
        out[1..21].copy_from_slice(&self.to);
        out[21..29].copy_from_slice(&self.nonce.to_le_bytes());
        out[29..37].copy_from_slice(&self.gas_limit.to_le_bytes());
        out[37..53].copy_from_slice(&self.max_fee.to_le_bytes());
        out[53..].copy_from_slice(&self.max_priority.to_le_bytes());
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let b: &[u8; QUOTE_LEN] = b.try_into().ok()?;
        Some(Quote {
            kind: b[0],
            to: b[1..21].try_into().ok()?,
            nonce: u64::from_le_bytes(b[21..29].try_into().ok()?),
            gas_limit: u64::from_le_bytes(b[29..37].try_into().ok()?),
            max_fee: u128::from_le_bytes(b[37..53].try_into().ok()?),
            max_priority: u128::from_le_bytes(b[53..].try_into().ok()?),
        })
    }
}

/// The gas limit settle quotes for an estimate: 6/5 of it, None on overflow.
pub fn padded(estimate: u64) -> Option<u64> {
    estimate.checked_mul(6).map(|g| g / 5)
}
