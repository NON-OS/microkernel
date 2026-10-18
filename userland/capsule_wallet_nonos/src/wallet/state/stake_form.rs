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
//! The stake form: stake or unstake, the amount typed, the position and
//! lock chosen, and the transaction its review shows.

use crate::wallet::send::Draft;

pub struct StakeForm {
    /// Stake (0) or unstake (1), picked by pointer.
    pub unstake: u8,
    /// NOX to stake, in wei. Held at chain precision rather than whole tokens
    /// so any amount can be typed, fractions included, with no ceiling beyond
    /// what the wallet actually holds.
    pub amount: u128,
    /// Decimal entry for the amount above: digits typed, places after the
    /// point, and whether a point has been started.
    pub digits: u32,
    pub places: u32,
    pub point: bool,
    /// Which staked position the Unstake tab acts on. The contract closes a
    /// position by index, not by amount.
    pub position: u64,
    /// Chosen lock term, as an index into the contract lock table. Zero is no
    /// lock, which is what plain stake() does.
    pub lock: u8,
    /// Two-step staking: 0 = needs the approve, 1 = ready to stake. Advances
    /// once the approve broadcasts and resets after the stake.
    pub step: u8,
    /// The staking transaction the stake screen reviews, made from fresh
    /// reads; Confirm signs exactly this. None while the form is up.
    pub draft: Option<Draft>,
}

impl StakeForm {
    pub fn new() -> Self {
        Self {
            unstake: 0,
            amount: 0,
            digits: 0,
            places: 0,
            point: false,
            position: 0,
            lock: 0,
            step: 0,
            draft: None,
        }
    }
}
