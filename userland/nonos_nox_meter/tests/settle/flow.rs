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

//! A job's flow, step by step: what it asks, signs and answers.

use nonos_nox_meter::flow::{Ask, Contracts, Flow, Next, Work};
use nonos_nox_meter::settle::{close_plan, topup_plan};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::{Close, TopUp};

use super::fixture::{gas, state, CHAIN, NOW, NOX, SETTLEMENT};
use super::ret::ret;

const AT: Contracts = Contracts { nox: NOX, settlement: SETTLEMENT, pool: [0; 20] };

fn signed(n: u8) -> Vec<Vec<u8>> {
    (1..=n).map(|i| vec![i; 3]).collect()
}

#[test]
fn a_topup_asks_gas_signs_two_and_broadcasts_each_in_order() {
    let t = TopUp { chain_id: CHAIN, channel_id: [1; 32], amount: 500 };
    let (mut f, first) = Flow::begin(Work::TopUp(t));
    assert_eq!(first, Next::Ask(Ask::Gas));
    let want = topup_plan(&state(), &t, (&NOX, &SETTLEMENT), &gas(), NOW).unwrap();
    assert_eq!(f.on_gas(&gas(), &state(), (&AT, NOW)), Ok(Next::Sign(want.to_vec())));
    assert_eq!(f.on_signed(signed(2)), Next::Ask(Ask::Broadcast(vec![1; 3])));
    assert_eq!(f.on_hash(&[0xa1; 32]), Next::Ask(Ask::Broadcast(vec![2; 3])));
    assert_eq!(f.on_hash(&[0xa2; 32]), Next::Done([[0xa1; 32], [0xa2; 32]].concat()));
    assert_eq!(f.work.chain_id(), CHAIN);
}

#[test]
fn a_close_reads_grace_first() {
    let c = Close { chain_id: CHAIN, channel_id: [1; 32] };
    let (mut f, first) = Flow::begin(Work::Close(c));
    assert_eq!(first, Next::Ask(Ask::View { view: 16, args: vec![] }));
    let late = state().channels[0].expiry + 3_600;
    let (mut early, _) = Flow::begin(Work::Close(c));
    assert_eq!(
        early.on_gas(&gas(), &state(), (&AT, late)),
        Err(MeterError::TooEarly),
        "no grace yet"
    );
    assert_eq!(f.on_view(ret(3_600)), Ok(Next::Ask(Ask::Gas)));
    assert_eq!(f.on_gas(&gas(), &state(), (&AT, late - 1)), Err(MeterError::TooEarly));
    let tx = close_plan(&state(), &c, &SETTLEMENT, &gas(), (late, 3_600)).unwrap();
    assert_eq!(f.on_gas(&gas(), &state(), (&AT, late)), Ok(Next::Sign(vec![tx])));
    assert_eq!(f.on_signed(signed(1)), Next::Ask(Ask::Broadcast(vec![1; 3])));
    assert_eq!(f.on_hash(&[0xc1; 32]), Next::Done(vec![0xc1; 32]));
}
