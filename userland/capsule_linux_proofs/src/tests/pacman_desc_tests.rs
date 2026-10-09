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

//! A repository `desc` record: every field an install uses, and every
//! record that could not be installed faithfully refused whole.

use crate::install::pacman::desc::records;

const SUM: &str = "5d41402abc4b2a76b9719d911017c592aaaabbbbccccddddeeeeffff00001111";

fn desc(filename: &str, sum: &str) -> String {
    format!(
        "%FILENAME%\n{filename}\n\n%NAME%\nnmap\n\n%VERSION%\n7.95-2\n\n%SHA256SUM%\n{sum}\n\n\
         %PGPSIG%\niQEzBAABCAAd\nFiEE\n\n%DEPENDS%\nglibc>=2.35\nlibpcap\nlua54=5.4.6\n\n\
         %PROVIDES%\nnmap-bin=7.95\n\n"
    )
}

#[test]
fn every_field_an_install_uses_is_read() {
    let r = records(&desc("nmap-7.95-2-x86_64.pkg.tar.zst", SUM)).expect("a whole record");
    assert_eq!((r.name.as_str(), r.version.as_str()), ("nmap", "7.95-2"));
    assert_eq!(r.filename, "nmap-7.95-2-x86_64.pkg.tar.zst");
    assert_eq!(r.sha256.map(|s| s[..2].to_vec()), Some(vec![0x5d, 0x41]));
    assert_eq!(r.pgpsig, "iQEzBAABCAAdFiEE", "a wrapped signature is joined");
    assert_eq!(r.depends, ["glibc", "libpcap", "lua54"], "constraints are dropped");
    assert_eq!(r.provides, ["nmap-bin"]);
}

#[test]
fn a_file_name_that_leaves_the_repository_is_refused() {
    for bad in ["../../etc/shadow", "a/b.pkg.tar.zst", "..", "."] {
        assert!(records(&desc(bad, SUM)).is_none(), "{bad}");
    }
}

#[test]
fn a_record_without_a_usable_checksum_is_refused() {
    for bad in ["", "abc", &SUM[1..], &SUM.replace('5', "g")] {
        assert!(records(&desc("x.pkg.tar.zst", bad)).is_none(), "{bad:?}");
    }
}
