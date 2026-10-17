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
//! A landed row's gas wait in unix seconds (v1.4.56): the withdraw started
//! at a unix now holds the button GAS_WAIT_S, as the wallet's clock counts.

use super::clock_tests::{NOW, RTC};
use crate::order::send_resume::GAS_WAIT_S;
use crate::shield::fresh_move::{reviewed, Moving, Reviewed};
use crate::shield::fresh_move_act::{act, Act};

const BODY: &str = "id=7\napproval=1\npieces=3\nshielded=36\nmax_network_fee=0.002\n\
     remainder=500\nshort_eth=5";

fn paid_at(t: u64) -> Moving {
    let review: Option<Reviewed> = reviewed(BODY);
    Moving { address: [9; 20], amount: 37, review, gas_paid_s: Some(t), ..Moving::default() }
}

#[test]
fn a_gas_withdraw_started_at_a_unix_now_holds_ten_minutes() {
    let m = paid_at(NOW);
    assert_eq!(act(Some(&m), NOW + GAS_WAIT_S - 1), Act::GasWaiting);
    assert_eq!(act(Some(&m), NOW + GAS_WAIT_S), Act::PayGas);
    assert_eq!(act(Some(&paid_at(RTC)), NOW), Act::GasWaiting, "an rtc mark holds for ever");
}
