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

//! A group joined only by a port without ACS, shared on a standard boot
//! (group_share.rs): the Gemini Lake pair that K11 refused.

use super::acs_refusal::shared_line;
use super::group_rule::may_share;
use super::laptop_tests::reach;
use super::laptops::*;

#[test]
fn the_gemini_lake_wifi_and_nvme_reach_each_other_through_their_ports() {
    let parts = glk();
    assert_eq!(reach(&parts, WIFI), [(RP4, RP0), (NVME, RP0)]);
    assert_eq!(reach(&parts, NVME), [(RP0, RP4), (WIFI, RP4)]);
}

#[test]
fn only_a_vendor_pair_on_a_standard_boot_of_a_posture_image_shares_a_reach() {
    assert!(may_share(true, false, true, true));
    for (built, hardened, vendor, reaches) in [
        (false, false, true, true),
        (true, true, true, true),
        (true, false, false, true),
        (true, false, true, false),
    ] {
        assert!(
            !may_share(built, hardened, vendor, reaches),
            "{built} {hardened} {vendor} {reaches}"
        );
    }
}

#[test]
fn a_shared_pair_is_said_with_member_hop_and_holder() {
    assert_eq!(
        shared_line(NVME, RP0, "driver.nvme0"),
        "shared: 02:00.0 reaches it through 00:1c.0 without ACS; driver.nvme0 holds it, each in its own domain"
    );
}
