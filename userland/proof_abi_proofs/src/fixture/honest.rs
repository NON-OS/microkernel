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

//! The honest container every verify test starts from, and the pins a
//! verifier of the fixture's listing would hold.

use nonos_proof_abi::{ctx, DeviceRoots, Pins};
use nox_verify::statements::ATTEST;

use super::{container, decode_policy, family, prove_bound, Family, Parts, DAY, LISTING};

pub fn honest() -> (Family, Parts, DeviceRoots) {
    let f = family();
    let rec = f.record.encode();
    let p = prove_bound(ctx(&rec));
    let parts = container(&rec, &p.section, &f.publics, &f.meas, &f.policy);
    (f, parts, p.roots)
}

pub fn pins(roots: &[DeviceRoots]) -> Pins<'_> {
    Pins {
        roots,
        listing_id: LISTING,
        days: (DAY - 1, DAY + 1),
        isolation: &ATTEST,
        policy: decode_policy,
    }
}
