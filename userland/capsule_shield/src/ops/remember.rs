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

//! Reading what a state request answers from the open store.

use std::sync::Arc;

use nox_shield_core::ffi::format_amount;

use super::held::OPEN;
use super::seen::{Seen, SEEN};

/// Take what a state request answers from the open store. On the worker,
/// after a job: here it may wait for the store, the service loop never does.
pub fn remember() {
    // The guard goes at the end of this statement: OPEN is taken again below.
    let Some(open) = OPEN
        .lock()
        .ok()
        .map(|o| o.as_ref().map(|o| (o.address.clone(), Arc::clone(&o.wallet), o.live)))
    else {
        return;
    };
    let seen = open.map(|(account, wallet, live)| {
        let address = wallet.receiving_address().ok();
        let fresh = wallet.fresh_withdrawal_address().map_err(|e| e.to_string());
        let ahead = wallet.fresh_ahead().unwrap_or_default();
        let history = wallet.history().map_err(|e| e.to_string()).map(|all| {
            all.iter()
                .map(|h| {
                    let tx = h.tx.as_deref().unwrap_or("");
                    crate::reply::entry(&h.kind, &h.coin, &h.amount, &h.stage, tx, h.at)
                })
                .collect()
        });
        let balances = wallet.balances().map_err(|e| e.to_string()).map(|all| {
            all.iter()
                .map(|b| {
                    let name = format!("{:?}", b.coin).to_uppercase();
                    format!(
                        "{name} {} {} {}",
                        format_amount(b.coin, b.spendable),
                        format_amount(b.coin, b.pending),
                        b.note_count
                    )
                })
                .collect()
        });
        Seen { account, wallet, address, balances, live, fresh, ahead, history }
    });
    let Ok(mut slot) = SEEN.lock() else { return };
    // Locked while this ran: what was read belongs to a store no longer open.
    let still = OPEN.lock().ok().and_then(|o| o.as_ref().map(|o| Arc::clone(&o.wallet)));
    *slot = seen.filter(|s| still.is_some_and(|w| Arc::ptr_eq(&w, &s.wallet)));
}
