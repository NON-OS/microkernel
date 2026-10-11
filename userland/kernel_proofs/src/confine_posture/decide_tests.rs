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

//! K1, fail closed (plan/50 P2): the claim's verdict for each device a
//! machine can present, without the opt-in, with it, and with it beside a
//! production build, which never honours it.

use super::decide::{opted_in, physical_dma, verdict, Verdict};
use crate::memory::iommu::dev_unconfined::posture_tail;
use Verdict::{Confine, NoDma, Refuse, Unconfined};

/// (requester id, covered, domain) for each case, and its verdict with no
/// opt-in and under the opt-in in a development build.
/// A device as the claim sees it: requester id, covered, domain.
type Device = (bool, bool, Result<(), bool>);

const CASES: [(Device, Verdict, Verdict); 6] = [
    ((true, true, Ok(())), Confine, Confine),
    // A unit covers it but gave no domain: refused, opt-in or not.
    ((true, true, Err(false)), Refuse, Refuse),
    // No unit in service at all: an AMD machine, VT-d off, a failed bring-up.
    ((true, true, Err(true)), Refuse, Unconfined),
    // No unit in service covers it.
    ((true, false, Ok(())), Refuse, Unconfined),
    ((true, false, Err(true)), Refuse, Unconfined),
    // No requester id: no bus master; the claim goes ahead, its DMA does not.
    ((false, false, Ok(())), NoDma, Unconfined),
];

#[test]
fn a_device_no_unit_confines_is_refused_without_the_opt_in() {
    for ((rid, covered, domain), closed, _) in CASES {
        assert_eq!(verdict(rid, covered, domain, false, false), closed, "{rid} {covered}");
    }
}

#[test]
fn the_opt_in_opens_only_a_development_build() {
    for ((rid, covered, domain), closed, open) in CASES {
        assert_eq!(verdict(rid, covered, domain, true, false), open, "{rid} {covered}");
        assert_eq!(verdict(rid, covered, domain, true, true), closed, "production");
    }
    assert!(opted_in(true, false));
    assert!(!opted_in(true, true) && !opted_in(false, false) && !opted_in(false, true));
}

#[test]
fn a_build_hands_out_physical_addresses_only_under_the_opt_in() {
    let open = cfg!(feature = "nonos-dev-unconfined-dma")
        && !cfg!(any(feature = "nonos-production", feature = "nonos-release"));
    assert_eq!(physical_dma(), open);
    let tail: &[u8] = if open { b", dev-unconfined-dma=1" } else { b"" };
    assert_eq!(posture_tail(), tail, "the posture line names the opt-in");
}
