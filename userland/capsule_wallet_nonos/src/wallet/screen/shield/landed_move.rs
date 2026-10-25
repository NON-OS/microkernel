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
//! What the landed rows say (v1.4.56): each address where money was found,
//! its NOX and ETH, and while its NOX moves into the Shield, the review and
//! one button: move, confirm, or pay the gas the address lacks.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::wallet::order::fees::exact;
use crate::wallet::shield::fresh_move::Moving;
use crate::wallet::shield::landed_held::Held;
use crate::wallet::state::State;

pub const ROW_MOVE: u8 = 10;
pub const MOVE_MAX: u8 = 6;

/// The landed addresses, due ones first, at most MOVE_MAX.
pub fn landed(state: &State) -> Vec<([u8; 20], Held)> {
    let l = &state.meter.landed;
    let due =
        l.due.entries.iter().filter_map(|e| e.held.filter(|h| h.any()).map(|h| (e.address, h)));
    due.chain(l.scan.found.iter().map(|f| (f.1, f.2))).take(MOVE_MAX.into()).collect()
}

pub(super) fn said(m: &Moving) -> Vec<String> {
    let (mut out, coin) = (Vec::new(), if m.eth { "ETH" } else { "NOX" });
    if let Some(r) = &m.review {
        match &r.refusal {
            Some(why) => out.push(why.clone()),
            None => out.push(format!(
                "{} standard deposits: {} {coin} reaches the Shield, up to {} ETH in network fees.{}",
                r.pieces,
                r.shielded,
                r.max_network_fee,
                if r.approval { " First the pool is approved for exactly their sum." } else { "" }
            )),
        }
        if r.short_eth > 0 {
            out.push(format!("This address lacks {} ETH for the gas.", exact(r.short_eth)));
        }
        if r.remainder > 0 {
            let left = exact(r.remainder);
            out.push(format!(
                "{left} {coin} is below the smallest standard deposit and stays here."
            ));
        }
    }
    if !m.sent.is_empty() {
        out.push(format!("{} sent.", m.sent.len()));
    }
    out.extend(m.stopped.clone());
    out
}
