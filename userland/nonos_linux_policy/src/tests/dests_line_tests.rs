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

//! `dests=` on a line of policies.txt: written in any order, kept canonical.

use super::dests_write::{entry, list, name, v4};
use super::line_tests::line::parse;

fn dests(line: &str) -> Result<Option<Vec<u8>>, String> {
    parse(line).map(|l| l.and_then(|l| l.dests))
}

#[test]
fn a_list_is_written_canonically_whatever_its_order() {
    let mut net = [0u8; 16];
    net[..4].copy_from_slice(&[0x20, 0x01, 0x0d, 0xb8]);
    let want = list(vec![
        v4(24, (443, 443), [10, 1, 2, 0]),
        name("api.example.org", (8000, 8080)),
        entry(6, 32, (1, 65535), &net),
    ]);
    let items = "api.example.org:8000-8080,[2001:db8::]/32:1-65535,10.1.2.0/24:443";
    assert_eq!(dests(&format!("/bin/p NET_ANON dests={items}")), Ok(Some(want.clone())));
    let swapped = "10.1.2.0/24:443,[2001:db8::]/32:1-65535,api.example.org:8000-8080";
    assert_eq!(dests(&format!("/bin/p dests={swapped} NET_ANON")), Ok(Some(want)));
    assert_eq!(dests("/bin/p NET_ANON"), Ok(None));
}

#[test]
fn a_list_the_canonical_form_refuses_fails_the_line() {
    for items in [
        "10.1.2.1/24:443",
        "example.org",
        "example.org:0",
        "example.org:90-80",
        "Example.org:443",
        "10.0.0.0/33:443",
        "[::ffff:1.2.3.4]/128:443",
        "a.org:443,a.org:443",
        "",
        "999.0.0.0/8:443",
    ] {
        assert!(dests(&format!("/bin/p NET_ANON dests={items}")).is_err(), "{items:?}");
    }
}
