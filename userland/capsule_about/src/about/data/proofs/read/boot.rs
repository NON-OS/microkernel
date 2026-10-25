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

//! The boot chain's marks and the route the policy store chose.

use super::super::session::{Boot, Mark};
use crate::about::data::verify::{recorded, Verdict};
use nonos_policy_proto::Field;

pub(super) fn boot() -> Boot {
    let mark = |v: Verdict| match v {
        Verdict::Holds => Mark::Holds,
        Verdict::Broken => Mark::Broken,
        Verdict::Unknown => Mark::Unknown,
    };
    match recorded() {
        Some(r) => Boot {
            signature: mark(r.kernel_signature),
            attestation: mark(r.attestation),
            proof: mark(r.proof),
        },
        None => Boot { signature: Mark::Unknown, attestation: Mark::Unknown, proof: Mark::Unknown },
    }
}

pub(super) fn chosen_route() -> Option<u8> {
    let port = nonos_policy_client::lookup()?;
    nonos_policy_client::get_u8(port, Field::NetworkRoute)
}
