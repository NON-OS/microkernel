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

//! A glibc program from the NONOS mirror: its listing reaches the Debian
//! family as a tool, and the pins its content carries are read whole or not
//! at all (install/deb/pinned.rs, as asset_needs.py writes them).

use crate::family::{split, Family};
use crate::install::deb::pinned::{needs_path, pinned, MAX_NEEDS};
use crate::install::tools::tool;
use crate::listing_family::package_arg;

const SUM: &str = "b1c7d1a0f00a5d3c9e2f2b6f2a6e1f0c4d1e9b8a7c6d5e4f3a2b1c0d9e8f7a6b";
const GMP: &str = "libgmp10 2:6.3.0+dfsg-3 pool/main/g/gmp/libgmp10_6.3.0+dfsg-3_amd64.deb";

#[test]
fn a_kali_tool_listing_is_a_tool_in_debians_family() {
    let arg = package_arg("kali.nonos-simplex-chat").expect("a usable tail");
    let (family, pkg) = split(&arg);
    assert_eq!((family, pkg), (Family::Debian, "nonos-simplex-chat"));
    assert_eq!(tool(pkg), Some("simplex-chat"));
    assert_eq!(needs_path("simplex-chat"), "usr/share/nonos-debs/simplex-chat");
}

#[test]
fn each_line_pins_a_pool_file_by_its_sha256() {
    let text =
        format!("{GMP} {SUM}\nlibc6 2.41-12 pool/main/g/glibc/libc6_2.41-12_amd64.deb {SUM}\n");
    let got = pinned(&text).expect("two good lines");
    assert_eq!(got.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), ["libgmp10", "libc6"]);
    assert_eq!(got[0].version, "2:6.3.0+dfsg-3");
    assert_eq!(got[0].filename, "pool/main/g/gmp/libgmp10_6.3.0+dfsg-3_amd64.deb");
    assert_eq!(got[1].sha256.map(|s| s[0]), Some(0xb1));
    assert!(got.iter().all(|r| r.depends.is_empty()), "a pin brings nothing more");
}

#[test]
fn one_bad_line_refuses_the_whole_file() {
    let bad = [
        String::new(),
        format!("{GMP} {}", &SUM[1..]),
        format!("{GMP} {SUM} extra"),
        format!("{GMP}  {SUM}"),
        format!("{GMP} {SUM}\n{GMP} {SUM}"),
        format!("Libgmp10 1 pool/main/g/gmp/x.deb {SUM}"),
        format!("-gmp 1 pool/main/g/gmp/x.deb {SUM}"),
        format!("g 1 pool/main/g/gmp/x.deb {SUM}"),
        format!("libgmp10 1 pool/../../etc/x.deb {SUM}"),
        format!("libgmp10 1 pool/main/g/gmp/x.tar {SUM}"),
        format!("libgmp10 1\u{7f} pool/main/g/gmp/x.deb {SUM}"),
        format!("{GMP} {SUM}\nlibc6 2.41 pool/main/g/glibc/libc6.deb"),
    ];
    for text in bad {
        assert!(pinned(&text).is_none(), "{text:?}");
    }
}

#[test]
fn past_the_bound_the_file_is_refused() {
    let line = |i: usize| format!("lib{i:03} 1 pool/main/l/lib/lib{i}.deb {SUM}\n");
    assert!(pinned(&(0..MAX_NEEDS).map(line).collect::<String>()).is_some());
    assert!(pinned(&(0..=MAX_NEEDS).map(line).collect::<String>()).is_none());
}
