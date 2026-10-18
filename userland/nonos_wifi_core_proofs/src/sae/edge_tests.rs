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

//! The edges: a scalar of exactly r and the identity element are refused; a
//! commit or refusal naming another group (a weaker one too) ends the
//! exchange, not followed; hunting and pecking runs its fixed rounds only.

use crate::hex::h;
use crate::sae::vectors::{ADDR1, ADDR2, PEER_COMMIT, PW};
use nonos_wifi_core::sae::frame::parse_commit;
use nonos_wifi_core::sae::group::ORDER;
use nonos_wifi_core::sae::hnp::{derive_pwe, derive_pwe_in};
use nonos_wifi_core::sae::{SaeFailure, SaeStation};

#[test]
fn a_scalar_of_r_and_the_identity_element_are_refused() {
    let good = h(PEER_COMMIT);
    let mut s = good.clone();
    s[2..34].copy_from_slice(&ORDER);
    assert!(parse_commit(&s).is_none(), "the scalar r itself");
    let mut below = ORDER;
    below[31] -= 1;
    s[2..34].copy_from_slice(&below);
    assert!(parse_commit(&s).is_some(), "r - 1 is a scalar");
    let mut e = good.clone();
    e[34..98].copy_from_slice(&[0; 64]);
    assert!(parse_commit(&e).is_none(), "the identity has no x and y");
}

#[test]
fn another_group_is_refused_not_followed() {
    let pwe = derive_pwe(PW, &[1u8; 14], &ADDR1, &ADDR2).unwrap();
    for group in [1u16, 2, 5, 20, 21] {
        let mut sta = SaeStation::start(pwe, false, &[0x11; 32], &[0x22; 32]).unwrap();
        let mut c = h(PEER_COMMIT);
        c[..2].copy_from_slice(&group.to_le_bytes());
        sta.on_frame(1, 0, &c);
        assert_eq!((sta.failure(), sta.pmk()), (Some(SaeFailure::BadCommit), None), "{group}");
    }
    // The AP refusing group 19 asks for another; there is none to offer.
    let mut sta = SaeStation::start(pwe, false, &[0x11; 32], &[0x22; 32]).unwrap();
    sta.on_frame(1, 77, &20u16.to_le_bytes());
    assert_eq!(sta.failure(), Some(SaeFailure::GroupNotSupported));
}

#[test]
fn hunting_and_pecking_runs_its_rounds_and_no_more() {
    let decoy = |pw: &[u8]| vec![7u8; pw.len()];
    let (mut short, mut found) = (0, 0);
    for n in 0..64u8 {
        let pw = [b'p', b'w', n];
        let full = derive_pwe(&pw, &decoy(&pw), &ADDR1, &ADDR2).expect("40 rounds find it");
        assert!(derive_pwe_in(&pw, &decoy(&pw), &ADDR1, &ADDR2, 40) == Some(full));
        match derive_pwe_in(&pw, &decoy(&pw), &ADDR1, &ADDR2, 1) {
            Some(p) => (found += 1, assert!(p == full, "the same element")).1,
            None => short += 1,
        }
    }
    assert!(short > 0 && found > 0, "one round finds some and not others: {found}/{short}");
}
