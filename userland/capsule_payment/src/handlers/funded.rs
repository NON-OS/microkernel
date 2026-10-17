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

//! FUNDED (op 21, plan/21), for app.store: whether one more install of a
//! voucher listing lands on a channel this meter holds, one byte. The body
//! names the chain and the market's listing string; the price and the
//! channel's listing id come from the market's commerce record for it, the
//! one PAY reads (`clients::offer`), never from the caller.
//!
//! Every way to not know answers FUNDED_NO under status 0: no key or state
//! (PAY's ENOPERSIST), no NOX on the chain, the market silent or refusing,
//! a record PAY would refuse, no row with room. A distinct status for each
//! would tell the store more than the bit. Only a body that does not parse
//! is EINVAL, which says nothing of the meter.

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract};
use nonos_nox_meter::pay::funded as takes_one;
use nonos_nox_meter::wire::row;
use nonos_payment_proto::{FundedAsk, EINVAL, FUNDED_NO, FUNDED_YES};

use crate::clients::offer;
use crate::meter::{now_ms, Meter};
use crate::server::Call;

pub fn funded(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    let Some(ask) = FundedAsk::parse(req.body) else {
        return req.answer(EINVAL, &[]);
    };
    let bit = if held(meter, &ask) { FUNDED_YES } else { FUNDED_NO };
    req.answer(0, &[bit])
}

fn held(meter: &mut Meter, ask: &FundedAsk) -> bool {
    // PAY's step 1 (pay.rs): with no key there is no state and no charge.
    if meter.ready().is_err() {
        return false;
    }
    let Ok(nox) = address(ask.chain_id, Contract::NoxToken) else {
        return false;
    };
    // The listing's default release, as the store prices and posts it.
    let Ok(o) = offer(&ask.listing, b"") else {
        return false;
    };
    let rows = meter.state.channels.iter().map(row);
    takes_one(rows, &o, ask.chain_id, &nox, now_ms() / 1000)
}
