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
 * An escrow order as lines a person reads (B8, plan/24, plan/29): its state
 * by the escrow's own numbers, what was funded and what still waits at the
 * deposit address. Pure: the NOX figures come through `nox`, so
 * wallet_proofs reads real replies with it.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_payment_proto::OrderStatus;

pub fn state_name(state: u8) -> &'static str {
    match state {
        0 => "not funded yet",
        1 => "funded: waiting for the seller to deliver",
        2 => "delivered: check it before the dispute window ends",
        3 => "disputed: the seller's bond answers for it",
        4 => "released to the seller",
        5 => "refunded",
        _ => "a state this wallet does not know",
    }
}

pub fn status_rows(s: &OrderStatus, nox: fn(u128) -> String) -> Vec<(&'static str, String)> {
    let mut rows = Vec::from([
        ("order", String::from(state_name(s.state))),
        ("funded", format!("{} NOX", nox(s.funded))),
    ]);
    if s.pending > 0 {
        rows.push((
            "waiting",
            format!("{} NOX at the deposit address, not yet in the order", nox(s.pending)),
        ));
    }
    rows
}
