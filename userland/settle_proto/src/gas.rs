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

//! OP_GAS. Request: chain_id u64 | address[20]. Reply body: balance u128
//! (wei) | tx_nonce u64 (the address's next) | max_fee_per_gas u128
//! | max_priority_fee_per_gas u128. Suggestions only: the signer bounds them.

pub const GAS_LEN: usize = 16 + 8 + 16 + 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Gas {
    pub balance: u128,
    pub tx_nonce: u64,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: u128,
}

impl Gas {
    pub fn request(chain_id: u64, address: &[u8; 20]) -> [u8; 28] {
        let mut out = [0u8; 28];
        out[..8].copy_from_slice(&chain_id.to_le_bytes());
        out[8..].copy_from_slice(address);
        out
    }

    pub fn encode(&self) -> [u8; GAS_LEN] {
        let mut out = [0u8; GAS_LEN];
        out[..16].copy_from_slice(&self.balance.to_le_bytes());
        out[16..24].copy_from_slice(&self.tx_nonce.to_le_bytes());
        out[24..40].copy_from_slice(&self.max_fee_per_gas.to_le_bytes());
        out[40..].copy_from_slice(&self.max_priority_fee_per_gas.to_le_bytes());
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let b: &[u8; GAS_LEN] = b.try_into().ok()?;
        Some(Gas {
            balance: u128::from_le_bytes(b[..16].try_into().ok()?),
            tx_nonce: u64::from_le_bytes(b[16..24].try_into().ok()?),
            max_fee_per_gas: u128::from_le_bytes(b[24..40].try_into().ok()?),
            max_priority_fee_per_gas: u128::from_le_bytes(b[40..].try_into().ok()?),
        })
    }
}
