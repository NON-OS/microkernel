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

/*
 * The Shield withdraw that funds a made escrow order (B8, the plan): the
 * block's price written exactly, in NOX, to the deposit address the meter
 * computed. Only a price the Shield quoted is funded; nothing the store
 * named goes into it.
 */

use alloc::string::String;

use super::fees_kind::{exact, Fees};
use super::pending_step::Step;
use super::read::Named;
use crate::wallet::state::shield_ui::ASSET_NOX;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FundAsk {
    pub asset: u8,
    pub amount: String,
    pub to: [u8; 20],
}

/// The withdraw Fund sets up, if the purchase is at its funding step.
pub fn fund_ask(step: &Step, read: &Result<Named, &'static str>, fees: &Fees) -> Option<FundAsk> {
    let (Step::Fund(m), Ok(Named::Escrow(b)), Fees::Quoted(_)) = (step, read, fees) else {
        return None;
    };
    Some(FundAsk { asset: ASSET_NOX, amount: exact(b.amount), to: m.deposit })
}
