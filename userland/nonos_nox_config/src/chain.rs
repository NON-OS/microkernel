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

//! The two chains the OS signs for. Production is mainnet; Sepolia is where
//! the contracts are tried first. Anything else is refused by every lookup.

pub const MAINNET: u64 = 1;
pub const SEPOLIA: u64 = 11_155_111;

// The time between blocks on each chain, in ms: both are proof of stake at
// 12 s slots. settle's log reads stop at 20 years of these blocks past a
// contract's deploy block (plan/26), so a chain's cap follows its block time.
pub const MAINNET_BLOCK_MS: u64 = 12_000;
pub const SEPOLIA_BLOCK_MS: u64 = 12_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Chain {
    Mainnet,
    Sepolia,
}

impl Chain {
    pub const fn id(self) -> u64 {
        match self {
            Chain::Mainnet => MAINNET,
            Chain::Sepolia => SEPOLIA,
        }
    }

    pub const fn block_ms(self) -> u64 {
        match self {
            Chain::Mainnet => MAINNET_BLOCK_MS,
            Chain::Sepolia => SEPOLIA_BLOCK_MS,
        }
    }

    pub const fn from_id(id: u64) -> Option<Chain> {
        match id {
            MAINNET => Some(Chain::Mainnet),
            SEPOLIA => Some(Chain::Sepolia),
            _ => None,
        }
    }
}
