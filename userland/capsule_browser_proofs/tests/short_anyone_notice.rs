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

//! A page reached by a short .anyone name says so next to the address, in
//! net.anon's own words; a full onion style address and every other host
//! say nothing. The toast it is drawn in is proved in popup_controls.

use capsule_browser_proofs::browser::short_name::{notice_for, SHORT_NOTICE};

const FULL: &str = "http://iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone/";

#[test]
fn a_short_name_carries_the_notice() {
    assert_eq!(notice_for("http://shield.anyone/"), Some(SHORT_NOTICE));
    assert_eq!(notice_for("https://shield.anyone/pay?x=1"), Some(SHORT_NOTICE));
}

#[test]
fn a_full_address_and_other_hosts_carry_none() {
    assert_eq!(notice_for(FULL), None);
    assert_eq!(notice_for("https://example.com/"), None);
    assert_eq!(notice_for("http://shield.example/"), None);
    assert_eq!(notice_for("not a url"), None);
}
