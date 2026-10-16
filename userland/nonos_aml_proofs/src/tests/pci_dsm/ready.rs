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

//! The wait selector over every way back: firmware's duration is taken
//! only for a wait it has a field for and only when shorter than the
//! ceiling, no wait is ever longer than its ceiling, and each wait reads
//! its own field.

use crate::pci_dsm::*;

const ALL: [Wake; 6] =
    [Wake::FromD1, Wake::FromD2, Wake::FromD3Hot, Wake::FromD3Cold, Wake::Reset, Wake::Flr];
pub const EDGES: [u32; 11] = [0, 1, 199, 200, 201, 9_999, 10_000, 10_001, 99_999, 100_001, u32::MAX];

fn ceiling(w: Wake) -> u32 {
    match w {
        Wake::FromD1 => 0,
        Wake::FromD2 => D2_TO_D0_MAX_US,
        Wake::FromD3Hot => D3HOT_TO_D0_MAX_US,
        Wake::FromD3Cold | Wake::Reset => RESET_MAX_US,
        Wake::Flr => FLR_MAX_US,
    }
}

fn every(us: u32) -> Readiness {
    Readiness { reset_us: us, link_up_us: us, flr_us: us, d3hot_to_d0_us: us, vf_enable_us: us }
}

#[test]
fn never_longer_than_the_ceiling_and_firmware_shortens_only() {
    for w in ALL {
        assert_eq!(ready_wait_us(None, w), ceiling(w), "{w:?}: no value, the ceiling");
        for us in EDGES {
            let got = ready_wait_us(Some(every(us)), w);
            assert!(got <= ceiling(w), "{w:?} {us}");
            let has_field = !matches!(w, Wake::FromD1 | Wake::FromD2);
            assert_eq!(got, if has_field { us.min(ceiling(w)) } else { ceiling(w) }, "{w:?} {us}");
        }
    }
}

#[test]
fn each_wait_reads_its_own_field() {
    let r = Readiness { reset_us: 7, link_up_us: 1, flr_us: 2, d3hot_to_d0_us: 3, vf_enable_us: 4 };
    let got = ALL.map(|w| ready_wait_us(Some(r), w));
    assert_eq!(got, [0, D2_TO_D0_MAX_US, 3, 7, 7, 2], "D1, D2, D3hot, D3cold, reset, FLR");
}
