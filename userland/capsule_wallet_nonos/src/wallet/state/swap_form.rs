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
//! The swap form: the pair, the amount typed, the pool's last quote and
//! the reader's slippage.

use crate::wallet::swap::Quote;

pub struct SwapForm {
    /// Which token the trade pays out of, as an index into the token list.
    pub from: u8,
    /// Which token the trade buys.
    pub to: u8,
    /// Amount to pay, in the paying token's smallest unit.
    pub amount_in: u128,
    /// What the pool last said this trade returns.
    pub quote: Quote,
    /// Slippage the reader will accept, in hundredths of a percent.
    pub slippage_bps: u32,
    /// Zero while the router still needs an allowance, one once it has one.
    pub step: u8,
    /// How many digits the reader has typed, so a correction knows what to
    /// take back.
    pub digits: u32,
    /// Digits typed after the point.
    pub places: u32,
    /// Whether the reader has started a fraction.
    pub point: bool,
    /// The no-price banner, dismissed until the amount changes.
    pub note_hidden: bool,
}

impl SwapForm {
    pub fn new() -> Self {
        Self {
            /*
             * NOX out, USDC in: the asset this wallet is for is the one a
             * reader most often pays with.
             */
            from: 0,
            to: 2,
            amount_in: 0,
            quote: Quote::default(),
            /*
             * Half a percent, which is where a pool with real depth sits. A
             * tolerance set high enough to always succeed protects nobody.
             */
            slippage_bps: 50,
            step: 0,
            digits: 0,
            places: 0,
            point: false,
            note_hidden: false,
        }
    }
}
