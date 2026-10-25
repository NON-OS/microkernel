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

//! H-02b: a port-only claim (port_only.rs) that no unit can confine goes
//! ahead with no domain and no DMA grant, in every posture, the opt-in
//! and a production build included; a capsule holding Dma or Mmio is not
//! port-only, and stays refused.

use super::decide::{verdict, Verdict};
use super::port_only::port_only;
use Verdict::{Confine, NoDma, Refuse};

/// Each posture a unit cannot confine the claim in: no unit in service,
/// none covering the device, a unit that gave it no domain; with and
/// without a requester id.
const UNCONFINED: [(bool, bool, Result<(), bool>); 5] = [
    (true, true, Err(true)),
    (true, false, Ok(())),
    (true, false, Err(true)),
    (true, true, Err(false)),
    (false, false, Ok(())),
];

/// (opt-in, production): none, development opt-in, the opt-in in production.
const BUILDS: [(bool, bool); 4] = [(false, false), (true, false), (true, true), (false, true)];

#[test]
fn port_only_gets_no_dma_wherever_no_unit_confines_it() {
    for (rid, covered, domain) in UNCONFINED {
        for (build, posture) in BUILDS.into_iter().flat_map(|b| [(b, false), (b, true)]) {
            let v = verdict(rid, covered, domain, port_only(false, false), build, posture);
            assert_eq!(v, NoDma, "{rid} {covered} {domain:?} {build:?} {posture}");
        }
    }
}

#[test]
fn port_only_under_a_unit_is_still_confined() {
    for build in BUILDS {
        assert_eq!(verdict(true, true, Ok(()), port_only(false, false), build, true), Confine);
    }
}

#[test]
fn a_capsule_holding_dma_or_mmio_is_refused_without_a_unit() {
    for (dma, mmio) in [(true, false), (false, true), (true, true)] {
        for (rid, covered, domain) in UNCONFINED.into_iter().filter(|d| d.0) {
            for build in [(false, false), (true, true), (false, true)] {
                let v = verdict(rid, covered, domain, port_only(dma, mmio), build, false);
                assert_eq!(v, Refuse, "dma {dma} mmio {mmio} {covered} {domain:?} {build:?}");
            }
        }
    }
}
