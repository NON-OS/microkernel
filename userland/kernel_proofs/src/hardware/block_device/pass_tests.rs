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

//! A selection pass that meets a block-wire capsule which does not answer
//! (the the maintainers' review): its other units are skipped for the rest of the
//! pass, other endpoints and other rows are still asked, and the wait for
//! the boot disk starts at the first refusal and is never moved later.

use core::sync::atomic::AtomicU64;

use super::pass::Pass;
use super::wait_since::{over, stamp};
use super::BlockDeviceError as E;

#[test]
fn a_silent_endpoints_other_units_are_skipped() {
    let mut p = Pass::default();
    assert!(!p.skips(Some(1)));
    p.refused(Some(1), E::TransportFailure);
    assert!(p.skips(Some(1)), "UFS's other seven units");
    assert!(!p.skips(Some(0)), "another endpoint");
    assert!(!p.skips(None), "a row off the block wire");
}

#[test]
fn only_a_transport_failure_silences_an_endpoint() {
    let mut p = Pass::default();
    for e in [E::NotReady, E::AccessDenied, E::Stale, E::ProtocolMismatch] {
        p.refused(Some(2), e);
        assert!(!p.skips(Some(2)), "{e:?} says nothing of the other units");
    }
    p.refused(None, E::TransportFailure);
    assert!(!p.skips(None));
    p.refused(Some(200), E::TransportFailure);
    assert!(!p.skips(Some(200)), "out of the mask, asked as before");
}

#[test]
fn the_wait_runs_from_the_first_refusal() {
    let since = AtomicU64::new(0);
    assert!(!over(&since, 40_000, 15_000), "no refusal yet");
    stamp(&since, 1_000);
    stamp(&since, 30_000);
    assert!(!over(&since, 15_999, 15_000));
    assert!(over(&since, 16_000, 15_000), "15 s after the first, not the last");
}
