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

//! The staking contract as it is actually deployed (NOXStakingV4 behind the
//! proxy `chain` names). Selectors are keccak of the signatures in the
//! verified ABI; the four read selectors this wallet already used live match
//! the same ABI byte for byte, which is what pins these to the real contract.

/// Read selector for an account's stake.
pub const SEL_GET_STAKE_INFO: [u8; 4] = [0xc3, 0x45, 0x31, 0x53];

/// Lock terms, in seconds, and the weight each earns in basis points. Ten
/// thousand is no boost, so a 365 day lock counts a stake at two and a half
/// times its size.
pub const LOCK_TERMS: [(u32, u32); 6] = [
    (0, 10_000),
    (30 * 86_400, 12_000),
    (60 * 86_400, 14_000),
    (90 * 86_400, 16_000),
    (180 * 86_400, 18_000),
    (365 * 86_400, 25_000),
];

/// Days a lock term runs, for labelling it.
pub fn lock_days(seconds: u32) -> u32 {
    seconds / 86_400
}
