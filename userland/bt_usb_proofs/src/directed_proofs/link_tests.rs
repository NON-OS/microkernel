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

//! A bonded keyboard that directs its advertising to an address this host
//! used before. Only a controller with the extended filter policies passes
//! that on (Vol 6, Part B, 4.3.3), as an LE Directed Advertising Report
//! (Vol 4, Part E, 7.7.65.11); the host takes it only if the target
//! resolves under its own IRK and the advertiser is bonded.

use super::fixture::{bonded, old_rpa};
use crate::air::PEER;
use crate::fixture::air;
use crate::link::find_hid;
use crate::private_tests::IRK;

#[test]
fn a_bonded_peer_aimed_at_an_old_address_of_ours_is_taken() {
    let (mut host, mut env, us) = bonded(true);
    air(&mut host).seen.directed_to = Some(old_rpa(&IRK));
    let peer = find_hid(&mut host, &mut env, &us).unwrap().expect("not taken");
    assert_eq!(peer.addr.0, PEER);
    assert_eq!(air(&mut host).seen.scan_filters.last(), Some(&0x02));
}

#[test]
fn without_the_feature_the_controller_drops_it_and_scans_with_policy_zero() {
    let (mut host, mut env, us) = bonded(false);
    air(&mut host).seen.directed_to = Some(old_rpa(&IRK));
    assert_eq!(find_hid(&mut host, &mut env, &us), Ok(None));
    assert_eq!(air(&mut host).seen.scan_filters.last(), Some(&0x00));
}
