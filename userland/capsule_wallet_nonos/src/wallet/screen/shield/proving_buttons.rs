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

//! The Proving screen's buttons and which earlier spend one settles.

use alloc::vec::Vec;
use shield_wire::*;

use crate::wallet::state::shield_ui::ShieldUi;

pub const STOP: u8 = 0;
pub const SETTLE: u8 = 1;
pub const TAKE_BACK: u8 = 2;
pub const SETTLE_EARLIER: u8 = 3;

/* The earliest kept spend its owner may settle now, by its number. */
pub fn earlier_to_settle(ui: &ShieldUi) -> Option<&str> {
    ui.earlier.iter().find(|s| s.self_settle && s.tx.is_none()).and_then(|s| s.id.as_deref())
}

/* The buttons this screen offers now, in footer order. */
pub fn buttons(ui: &ShieldUi) -> Vec<(&'static str, u8)> {
    if let Some(w) = ui.waiting {
        return if matches!(w.op, OP_SEND | OP_WITHDRAW) {
            Vec::from([("Stop", STOP)])
        } else {
            Vec::new()
        };
    }
    let mut out = Vec::new();
    if let Some(s) =
        ui.spend.as_ref().filter(|s| s.tx.is_none() && !s.state.starts_with("settling"))
    {
        if s.self_settle {
            out.push(("Settle it myself", SETTLE));
        }
        if !s.published || s.refusal.is_some() {
            out.push(("Take back", TAKE_BACK));
        }
    }
    if earlier_to_settle(ui).is_some() {
        out.push(("Settle an earlier one myself", SETTLE_EARLIER));
    }
    out.truncate(3);
    out
}
