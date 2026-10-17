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
use nonos_payment_proto::OrderMade;

use super::fixture::{new, ESCROW, FWD, NOW, NOX, SALT};
use super::host::Host;
use super::ret::{ret, word};

pub fn order() -> Order {
    order_plan(&Host, &new(NOW + ORDER_MIN_LEAD, 1_000), (&NOX, &ESCROW, &FWD), SALT, NOW).unwrap()
}

pub fn view(view: u16, args: Vec<[u8; 32]>) -> Next {
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
