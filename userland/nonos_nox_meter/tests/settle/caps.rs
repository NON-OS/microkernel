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

use nonos_nox_meter::settle::settle_plan;
use nonos_nox_meter::MeterError;
use nonos_payment_proto::DrainSign;

use super::fixture::{sign, state, METER, NOW, SETTLEMENT};

#[test]
fn sign_stays_under_the_fee_caps_and_the_gas_bound() {
    let s = state();
    for (fee, tip, gas) in [
        (1_000_000_000_001, 1, 1),
        (10, 10_000_000_001, 1),
        (10, 11, 1),
        (10, 2, 0),
        (10, 2, 3_000_001),
    ] {
        let req = DrainSign { max_fee: fee, max_priority: tip, gas_limit: gas, ..sign(4, &[1]) };
        assert_eq!(
            settle_plan(&s, &METER, &req, &SETTLEMENT, NOW).err(),
            Some(MeterError::NoGas),
            "{fee} {tip} {gas}"
        );
    }
}
