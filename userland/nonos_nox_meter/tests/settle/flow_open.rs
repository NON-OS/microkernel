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

//! OPEN START's flow: approve and open, never past the asked expiry.

use nonos_nox_meter::flow::{Ask, Contracts, Flow, Next, Work};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::OpenStart;
use nonos_settle_proto::Gas;

use super::fixture::{gas, state, CHAIN, NOW, NOX, SETTLEMENT};

const AT: Contracts = Contracts { nox: NOX, settlement: SETTLEMENT, pool: [0; 20] };

#[test]
fn an_open_signs_approve_and_open_unless_already_expired() {
    let o = OpenStart {
        chain_id: CHAIN,
        payee: [0xbb; 20],
        listing_id: [2; 32],
        unit: [3; 32],
        amount: 1_000,
        expiry: NOW + 86_400,
        reclaim_to: [0xcc; 20],
    };
    let (mut f, first) = Flow::begin(Work::Open(o));
    assert_eq!(first, Next::Ask(Ask::Gas));
    let g = Gas { balance: 260_000 * 10, ..gas() };
    match f.on_gas(&g, &state(), (&AT, NOW)) {
        Ok(Next::Sign(txs)) => assert_eq!((txs.len(), txs[0].to, txs[1].to), (2, NOX, SETTLEMENT)),
        other => panic!("{other:?}"),
    }
    let (mut late, _) = Flow::begin(Work::Open(o));
    assert_eq!(late.on_gas(&g, &state(), (&AT, o.expiry)), Err(MeterError::Expired));
}
