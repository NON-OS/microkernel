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

//! The kernel turns a listing's tail into the name the personality reads,
//! and the personality splits that name back into family and package. The
//! two ends are tested together, so they cannot drift apart.

use crate::family::{split, Family};
use crate::listing_family::package_arg;

#[test]
fn each_namespace_reaches_its_family_with_the_package_intact() {
    for (tail, family, pkg) in [
        ("jq", Family::Alpine, "jq"),
        ("kali.jq", Family::Debian, "jq"),
        ("kali.libc6", Family::Debian, "libc6"),
        ("blackarch.nmap", Family::Pacman, "nmap"),
        ("g++", Family::Alpine, "g++"),
    ] {
        let arg = package_arg(tail).expect(tail);
        assert_eq!(split(&arg), (family, pkg), "{tail}");
    }
}

#[test]
fn a_namespace_with_no_package_is_refused() {
    for tail in ["", ".", ".jq", "kali.", "kali..jq", "blackarch.", "blackarch..x"] {
        assert_eq!(package_arg(tail), None, "{tail:?}");
    }
}

#[test]
fn a_listing_cannot_smuggle_a_family_prefix_of_its_own() {
    // The personality would read `deb:jq` as Debian, so the kernel never
    // passes a tail with a colon in it, under any namespace.
    for tail in ["deb:jq", "pacman:nmap", "kali.deb:jq", "blackarch.deb:x", "jq:"] {
        assert_eq!(package_arg(tail), None, "{tail}");
    }
}
