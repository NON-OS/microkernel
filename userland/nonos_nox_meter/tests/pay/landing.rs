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
//! `landing` is the first pass pay_charge.rs made inline: MeterState's
//! charge with no usage root, its refusals as they were, then the unit,
//! a Mismatch (PAY's EPROTO) where charged alone would have taken it.

use nonos_nox_meter::budget::Budget;
use nonos_nox_meter::channel::Channel;
use nonos_nox_meter::pay::landing;
use nonos_nox_meter::state::MeterState;
use nonos_nox_meter::MeterError;
use nonos_payment_proto::PayRequest;

use super::fixture::request;
use super::funded_fixture::{channel, state, NOW};

fn inline(s: &MeterState, r: &PayRequest<'_>) -> Result<Channel, MeterError> {
    let (_, at) = s.charged(&r.payee, &r.listing_id, r.chain_id, 7, [0; 32], NOW)?;
    if at.unit != r.unit {
        return Err(MeterError::Mismatch);
    }
    Ok(at)
}

#[test]
fn landing_is_the_charge_pay_made_inline_then_the_unit() {
    let r = PayRequest { qty: 1, ..request() };
    for s in [
        state(&[channel()]),
        state(&[]),
        state(&[Channel { frozen: true, ..channel() }]),
        state(&[Channel { expiry: NOW, ..channel() }]),
        state(&[Channel { cumulative: 14, ..channel() }]),
        state(&[Channel { unit: [4; 32], ..channel() }]),
        broke(),
    ] {
        assert_eq!(landing(&s, &r, 7, NOW), inline(&s, &r), "{s:?}");
    }
    let s = state(&[channel()]);
    let (_, charged) = s.charged(&r.payee, &r.listing_id, 1, 7, [0; 32], NOW).unwrap();
    assert_eq!(landing(&s, &r, 7, NOW), Ok(charged));
    let odd = state(&[Channel { unit: [4; 32], ..channel() }]);
    assert!(odd.charged(&r.payee, &r.listing_id, 1, 7, [0; 32], NOW).is_ok());
    assert_eq!(landing(&odd, &r, 7, NOW), Err(MeterError::Mismatch));
    assert_eq!(landing(&state(&[]), &r, 7, NOW), Err(MeterError::NoChannel));
    assert_eq!(landing(&broke(), &r, 7, NOW), Err(MeterError::Expired));
}

fn broke() -> MeterState {
    MeterState { budget: Budget::default(), ..state(&[channel()]) }
}
