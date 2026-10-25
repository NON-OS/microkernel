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

//! The charge itself: the channel and the budget take it, with the usage
//! leaf naming this call, and the state reaches disk before anything is
//! signed from it.

use nonos_nox_meter::channel::Channel;
use nonos_nox_meter::merkle::root;
use nonos_nox_meter::pay::landing;
use nonos_nox_meter::usage::Charge;
use nonos_payment_proto::PayRequest;

use crate::keccak::Host;
use crate::meter::{now_ms, Meter};
use crate::server::status_of;

pub(super) fn charge(meter: &mut Meter, r: &PayRequest<'_>, amount: u128) -> Result<Channel, i32> {
    let now = now_ms() / 1000;
    let s = &meter.state;
    // The leaf names the channel and the seq the charge will have, which only
    // the charge decides; a first pass with no root finds them.
    // That pass refuses as FUNDED does: a channel's vouchers carry the unit
    // it was opened for, and no other (EPROTO).
    let at = landing(s, r, amount, now).map_err(status_of)?;
    let leaf = Charge {
        channel_id: at.id,
        seq: at.seq,
        unit: r.unit,
        qty: r.qty,
        price: r.unit_price,
        call_id: r.call_id,
    }
    .leaf(&Host);
    let usage = root(&Host, &[leaf]);
    let (next, c) =
        s.charged(&r.payee, &r.listing_id, r.chain_id, amount, usage, now).map_err(status_of)?;
    meter.commit(next)?;
    Ok(c)
}
