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

//! What a voucher listing's steps say first (B8, v1.4.38, v1.4.48).

use crate::wallet::order::held::Held;
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::voucher_form::Budgeted;

const ASKING: &str = "Asking the meter whether this listing already has a channel open.";
const VOUCHER: &str = "This listing is paid by voucher as it is used. Open a channel: set a \
     deposit and how long it runs; what is not spent comes back to a fresh address of this wallet.";
const HELD: &str = "This listing already has a channel open, below. Use it: the store pays \
     from it by voucher. A second channel is opened only if you ask for another.";
const OPENING: &str = "The meter is signing the channel's approval and opening.";
const CONFIRMING: &str = "Waiting for the chain to mine the channel's opening.";
const NO_BUDGET: &str = "The channel is open, but the meter has no budget in force, so it would \
     refuse to pay. Set one: the deposit, for 30 days, with this listing's cap.";
const OPENED: &str = "The channel is open. Install the app again from the store: it pays from \
     this channel by voucher.";
const KEPT: &str = "Nothing new was opened. Install the app again from the store: it pays from \
     the channel already open.";

/// the project and the action beside Close while a voucher listing is offered.
pub fn consent(held: Held) -> (&'static str, Option<&'static str>) {
    match held {
        Held::Asking(_) => (ASKING, None),
        Held::Open(_) => (HELD, Some("Use it")),
        Held::Free | Held::Another => (VOUCHER, Some("Open channel")),
    }
}

/// the project and the action once a channel is chosen or opening.
pub fn lead(p: &Pending) -> (&'static str, Option<&'static str>) {
    match p.step {
        Step::Opening(..) => (OPENING, None),
        Step::Confirming(..) => (CONFIRMING, None),
        Step::Opened(_) if p.form.budgeted == Budgeted::NoBudget => {
            (NO_BUDGET, Some("Set budget for 30 days"))
        }
        Step::Kept(_) => (KEPT, None),
        _ => (OPENED, None),
    }
}

/// The third footer action: open a second channel beside the open one.
pub fn another(p: &Pending) -> Option<&'static str> {
    let offered = matches!((&p.step, p.held), (Step::Consent, Held::Open(_)));
    offered.then_some("Open another")
}
