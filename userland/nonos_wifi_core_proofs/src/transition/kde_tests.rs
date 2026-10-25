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

//! The KDE walk reads the Transition Disable KDE under the Wi-Fi Alliance
//! OUI alone, keeps the first, and lets a short one fail nothing.

use super::sim::td_kde;
use nonos_wifi_core::eapol::kde::{parse_key_data, TD_WPA3_PERSONAL};

#[test]
fn the_first_octet_of_the_kde_is_read() {
    let one = td_kde(TD_WPA3_PERSONAL);
    let kd = parse_key_data(&one).unwrap();
    assert_eq!(kd.transition_disable, Some(TD_WPA3_PERSONAL));
    let wide = [0xDD, 6, 0x50, 0x6F, 0x9A, 0x20, 0x0f, 0xff];
    assert_eq!(parse_key_data(&wide).unwrap().transition_disable, Some(0x0f));
}

#[test]
fn only_the_first_kde_counts() {
    let kd = [td_kde(0x01), td_kde(0x0f)].concat();
    assert_eq!(parse_key_data(&kd).unwrap().transition_disable, Some(0x01));
}

#[test]
fn another_oui_or_type_is_not_the_indication() {
    let ieee = [0xDD, 5, 0x00, 0x0F, 0xAC, 0x20, 0x01];
    let other = [0xDD, 5, 0x50, 0x6F, 0x9A, 0x21, 0x01];
    for kd in [&ieee[..], &other[..]] {
        assert!(parse_key_data(kd).unwrap().transition_disable.is_none());
    }
}

#[test]
fn a_kde_with_no_bits_is_ignored_and_fails_nothing() {
    let empty = [0xDD, 4, 0x50, 0x6F, 0x9A, 0x20];
    let kd = parse_key_data(&empty).expect("the field still parses");
    assert!(kd.transition_disable.is_none());
    let then = [&empty[..], &td_kde(0x01)].concat();
    assert_eq!(parse_key_data(&then).unwrap().transition_disable, Some(0x01));
}
