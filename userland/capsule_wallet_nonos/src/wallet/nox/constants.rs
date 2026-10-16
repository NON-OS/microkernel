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

// Function selectors (first 4 bytes of keccak256 of the signature). Read paths
// only; the SDK's live/selectors.js is the source of record.
pub const SEL_BALANCE_OF: [u8; 4] = [0x70, 0xa0, 0x82, 0x31]; // balanceOf(address)
pub const SEL_PENDING_REWARDS: [u8; 4] = [0x31, 0xd7, 0xa2, 0x62]; // pendingRewards(address)
pub const SEL_ACTIVE_POSITIONS: [u8; 4] = [0x76, 0x9c, 0xba, 0x4b]; // activePositionCount(address)
pub const SEL_PROTOCOL_STATS: [u8; 4] = [0xa8, 0xbb, 0xac, 0x10]; // protocolStakingStats()

// Word offsets inside the protocolStakingStats() tuple return.
pub const STATS_TOTAL_STAKED: usize = 0;
pub const STATS_REWARDS_DISTRIBUTED: usize = 3;
pub const STATS_EMISSION_RATE: usize = 5;

pub const SECONDS_PER_YEAR: u128 = 31_536_000;
