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

//! ORDER_NEW's and ORDER_STATUS's flows on Sepolia: what each reads, its answer.

use nonos_nox_config::{Contract, ORDER_MIN_LEAD};
use nonos_nox_meter::flow::{Ask, Flow, Next, Work};
use nonos_nox_meter::orders::{order_plan, Order};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::{OrderMade, OrderStatus};

use super::fixture::{new, ESCROW, FWD, NOW, NOX, SALTS};
use super::host::Host;
use super::ret::{ret, word};

fn order() -> Order {
    order_plan(&Host, &new(NOW + ORDER_MIN_LEAD, 1_000), (&NOX, &ESCROW, &FWD), SALTS, NOW).unwrap()
}

fn view(view: u16, args: Vec<[u8; 32]>) -> Next {
    Next::Ask(Ask::View { view, args })
}

#[test]
fn a_new_order_reads_paused_then_its_deposit_address_and_is_kept() {
    let o = order();
    assert_eq!(Work::OrderNew(o).needs(), [Contract::NoxToken, Contract::Escrow]);
    let (mut f, first) = Flow::begin(Work::OrderNew(o));
    assert_eq!(first, view(72, vec![]));
    assert_eq!(f.on_view(ret(0)), Ok(view(21, vec![o.order_id])));
    let made = OrderMade { order_id: o.order_id, deposit: o.deposit, terms: o.terms };
    assert_eq!(f.on_view(vec![word(&o.deposit)]), Ok(Next::Keep(Box::new(o), made.encode())));
}

#[test]
fn a_paused_policy_or_another_deposit_address_ends_the_order() {
    let o = order();
    let step = |paused: u128, deposit: [u8; 20]| {
        let (mut f, _) = Flow::begin(Work::OrderNew(o));
        f.on_view(ret(paused)).and_then(|_| f.on_view(vec![word(&deposit)]))
    };
    assert_eq!(step(1, o.deposit), Err(MeterError::Paused));
    assert_eq!(step(2, o.deposit), Err(MeterError::Mismatch), "paused() is a strict bool");
    assert_eq!(step(0, [0xee; 20]), Err(MeterError::Untrusted));
}

#[test]
fn a_status_is_order_of_then_what_waits_at_the_deposit_address() {
    let o = order();
    let (mut f, first) = Flow::begin(Work::OrderStatus(o));
    assert_eq!(first, view(20, vec![o.order_id]));
    let of = [ret(2), ret(900), ret(NOW.into()), ret(0)].concat();
    assert_eq!(f.on_view(of.clone()), Ok(view(1, vec![word(&o.deposit)])));
    assert_eq!(f.on_view(ret(70)), Ok(view(60, vec![ret(1)[0], word(&o.deposit)])));
    let want = OrderStatus { state: 2, funded: 900, pending: 75 };
    assert_eq!(f.on_view(ret(5)), Ok(Next::Done(want.encode())));
    let (mut bad, _) = Flow::begin(Work::OrderStatus(o));
    let wide = [ret(256), of[1..].to_vec()].concat();
    let _ = bad.on_view(wide).and_then(|_| bad.on_view(ret(0)));
    assert_eq!(bad.on_view(ret(0)), Err(MeterError::Mismatch), "a state past u8");
}
