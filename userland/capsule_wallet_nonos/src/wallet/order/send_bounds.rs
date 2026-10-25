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

//! Settle's INTENT quote held to what its signer asked: the kind,
//! the config's contract, a gas limit, and the meter's fee caps. Settle only
//! suggests; anything out of bounds is refused rather than trimmed.

use nonos_nox_config::{MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS};
use nonos_settle_proto::Quote;

/// Why a quote is not signed: another kind, a contract other than `to`, gas
/// past `gas_max`, or fees past the caps. Shared with the publisher's intents.
pub fn held_to(q: &Quote, kind: u8, to: Option<[u8; 20]>, gas_max: u64) -> Option<&'static str> {
    if q.kind != kind {
        return Some("settle quoted another kind of transaction, not signing");
    }
    if to != Some(q.to) {
        return Some("settle quoted a contract other than the config's, not signing");
    }
    if q.gas_limit == 0 || q.gas_limit > gas_max {
        return Some("settle's gas limit is out of bounds, not signing");
    }
    if q.max_fee == 0 || q.max_fee > MAX_FEE_PER_GAS {
        return Some("network fee zero or above 1000 gwei, not signing");
    }
    if q.max_priority > q.max_fee || q.max_priority > MAX_PRIORITY_FEE_PER_GAS {
        return Some("the tip is above the fee or above 10 gwei, not signing");
    }
    None
}
