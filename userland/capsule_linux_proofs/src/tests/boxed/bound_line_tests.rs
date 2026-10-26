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

//! The line a store program's binding leaves: who vouched, the listing, the
//! mask named and the mask served, the served bits by name, no process.

use crate::app::bound_line::{bound_line, Vouched};
use nonos_linux_policy::mask::{KNOWN, NAMES, NET_ANON, TTY};

#[test]
fn a_market_binding_names_its_listing_and_what_it_is_served() {
    let line = bound_line(Vouched::Market, &[0xab; 32], NET_ANON | TTY, TTY);
    let want = alloc::format!(
        "[LINUX] policy bound: from=market listing={} mask=0x102 served=0x100 TTY\n",
        "ab".repeat(32)
    );
    assert_eq!(line, want);
}

#[test]
fn every_served_bit_is_named_once_and_the_line_fits_one_debug_call() {
    let line = bound_line(Vouched::Default, &[0xff; 32], KNOWN, KNOWN);
    for (name, _) in NAMES {
        assert_eq!(line.matches(&alloc::format!(" {name}")).count(), 1, "{name}");
    }
    assert!(line.len() <= 256, "{}", line.len());
}

#[test]
fn the_vendor_row_wins_over_a_file_as_choose_does() {
    assert_eq!(Vouched::of(true, true), Vouched::Vendor);
    assert_eq!(Vouched::of(true, false), Vouched::Vendor);
    assert_eq!(Vouched::of(false, true), Vouched::Market);
    assert_eq!(Vouched::of(false, false), Vouched::Default);
    for (v, word) in [
        (Vouched::Vendor, "from=vendor"),
        (Vouched::Market, "from=market"),
        (Vouched::Default, "from=default"),
    ] {
        assert!(bound_line(v, &[0; 32], 0, 0).contains(word), "{word}");
    }
}
