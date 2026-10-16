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

//! The readiness wait: firmware's duration is taken only when it is
//! shorter than the specification's ceiling, the wait is never longer
//! than the ceiling, and the sleep it becomes never cuts it short.

use crate::device_power::DState;
use crate::pci_dsm::*;

fn said(us: u32) -> Readiness {
    let (reset_us, d3hot_to_d0_us) = (us, us);
    Readiness { reset_us, link_up_us: 0, flr_us: 0, d3hot_to_d0_us, vf_enable_us: 0 }
}

const EDGES: [u32; 9] = [0, 1, 999, 1_000, 9_999, 10_000, 10_001, 99_999, u32::MAX];

#[test]
fn never_longer_than_the_ceiling() {
    for (w, ceiling) in [(Wake::FromD3Hot, D3HOT_TO_D0_MAX_US), (Wake::FromD3Cold, RESET_MAX_US)] {
        assert_eq!(ready_wait_us(None, w), ceiling, "no value: the ceiling");
        for us in EDGES.into_iter().chain([100_000, 100_001]) {
            assert!(ready_wait_us(Some(said(us)), w) <= ceiling, "{us} past {ceiling}");
        }
        assert_eq!(ready_wait_us(Some(said(u32::MAX)), w), ceiling, "all ones: the ceiling");
    }
}

#[test]
fn firmware_shortens_only() {
    for us in EDGES {
        let hot = ready_wait_us(Some(said(us)), Wake::FromD3Hot);
        let cold = ready_wait_us(Some(said(us)), Wake::FromD3Cold);
        assert_eq!(hot, us.min(D3HOT_TO_D0_MAX_US));
        assert_eq!(cold, us.min(RESET_MAX_US));
    }
    let r = Readiness { reset_us: 7, link_up_us: 1, flr_us: 2, d3hot_to_d0_us: 3, vf_enable_us: 4 };
    assert_eq!(ready_wait_us(Some(r), Wake::FromD3Hot), 3, "D3hot reads its own field");
    assert_eq!(ready_wait_us(Some(r), Wake::FromD3Cold), 7, "D3cold reads the reset field");
}

#[test]
fn only_the_ways_back_to_d0_wait() {
    use DState::*;
    let all = [D0, D1, D2, D3Hot, D3Cold];
    for from in all {
        for to in all {
            let w = wake_of(from, to);
            let want = match (from, to) {
                (D3Hot, D0) => Some(Wake::FromD3Hot),
                (D3Cold, D0) => Some(Wake::FromD3Cold),
                _ => None,
            };
            assert_eq!(w, want, "{from:?} -> {to:?}");
        }
    }
}

#[test]
fn the_sleep_never_cuts_the_wait_short() {
    for us in EDGES.into_iter().chain([D3HOT_TO_D0_MAX_US, RESET_MAX_US]) {
        let ms = sleep_ms_for(us);
        assert!(ms * 1000 >= u64::from(us) + 1000, "{us} us slept as {ms} ms");
        assert!(ms * 1000 < u64::from(us) + 2000, "{us} us slept as {ms} ms, too long");
    }
}
