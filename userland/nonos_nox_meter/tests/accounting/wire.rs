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

use super::fixture::{funded_state, LISTING, NOW, PAYEE, ROOT};
use nonos_nox_meter::wire::{account, budget, drain};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::{SetBudget, KEPT, READY};

#[test]
fn a_set_budget_body_is_a_fresh_allowance_or_refused() {
    let b = SetBudget { cap: 500, expiry: NOW + 9, listings: vec![(LISTING, 40)] };
    let got = budget(&b).unwrap();
    assert_eq!((got.cap, got.spent, got.expiry, got.listings[0].cap), (500, 0, NOW + 9, 40));
    let twice = SetBudget { listings: vec![(LISTING, 1), (LISTING, 2)], ..b };
    assert_eq!(budget(&twice), Err(MeterError::Duplicate));
}

#[test]
fn account_shows_the_address_budget_and_channel_rows() {
    let s = funded_state();
    let none = account(&s, None, false, NOW, 0);
    assert_eq!((none.flags, none.address, none.rows.len()), (0, [0; 20], 1));
    let a = account(&s, Some([0xaa; 20]), true, NOW, 0);
    assert_eq!((a.flags, a.address, a.cap, a.left), (READY | KEPT, [0xaa; 20], 500, 500));
    assert_eq!((a.total, a.rows[0].channel_id, a.rows[0].frozen), (1, s.channels[0].id, false));
    assert_eq!(a.rows[0].listing_id, s.channels[0].listing_id, "v1.4.48");
    let past = account(&s, None, true, NOW, 1);
    assert_eq!((past.flags, past.total, past.rows.len()), (KEPT, 1, 0));
}

#[test]
fn drain_hands_out_only_signed_vouchers() {
    let payer = [0xaa; 20];
    let (s, c) = funded_state().charged(&PAYEE, &LISTING, 1, 7, ROOT, NOW).unwrap();
    let d = drain(&s, &payer, 0);
    assert_eq!((d.total, d.rows.len()), (0, 0));
    let s = s.signed(&c.id, c.seq, [9; 65]);
    let d = drain(&s, &payer, 0);
    assert_eq!((d.total, d.rows.len()), (1, 1));
    let v = d.rows[0];
    assert_eq!((v.channel_id, v.payer, v.cumulative, v.sig), (c.id, payer, 7, [9; 65]));
    assert_eq!((v.expiry, v.usage_root), (c.expiry, ROOT));
}
