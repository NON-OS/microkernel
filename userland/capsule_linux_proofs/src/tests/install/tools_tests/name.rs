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

//! A tool's listing, its name, and where its content is served.

use crate::family::{split, Family};
use crate::install::tools::{content_path, tool, PREFIX};
use crate::listing_family::package_arg;

#[test]
fn a_tool_listing_reaches_the_personality_in_alpines_family() {
    for (tail, want) in [("nonos-rg", "rg"), ("nonos-perl", "perl"), ("nonos-openssl", "openssl")] {
        let arg = package_arg(tail).expect(tail);
        assert_eq!(arg, tail, "the kernel passes the name as it is");
        let (family, pkg) = split(&arg);
        assert_eq!(family, Family::Alpine, "{tail}");
        assert_eq!(tool(pkg), Some(want), "{tail}");
    }
}

#[test]
fn only_a_name_in_the_namespace_is_a_tool() {
    for pkg in ["rg", "jq", "qwen-small", "nonos", "nonosrg", "kali.rg"] {
        assert_eq!(tool(pkg), None, "{pkg}");
    }
    assert_eq!(PREFIX, "nonos-");
}

#[test]
fn a_tool_name_is_plain() {
    for pkg in ["nonos-", "nonos--x", "nonos-.x", "nonos-RG", "nonos-a/b", "nonos-a:b", "nonos-a b", "nonos-é"] {
        assert_eq!(tool(pkg), None, "{pkg:?}");
    }
    for (pkg, want) in [("nonos-gojq", "gojq"), ("nonos-a-b", "a-b"), ("nonos-v1.2", "v1.2"), ("nonos-0", "0")] {
        assert_eq!(tool(pkg), Some(want), "{pkg}");
    }
}

#[test]
fn the_content_is_served_at_its_pin() {
    let mut pin = [0u8; 32];
    pin[0] = 0xab;
    pin[31] = 0x01;
    let path = content_path(&pin);
    assert_eq!(path.len(), "/linux/".len() + 64 + ".tar".len());
    assert!(path.starts_with("/linux/ab00"));
    assert!(path.ends_with("0001.tar"));
    assert_eq!(content_path(&[0xff; 32]), alloc::format!("/linux/{}.tar", "f".repeat(64)));
}
