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

//! What a package may not write into the link table (place_links.rs): a
//! path with a space or a newline, or a target with a newline, each of which
//! would let it add a line of its own choosing; nor a table past its cap.
//! And an uninstall takes out its own lines and leaves the others.

use super::place_links::table::{MAX_TABLE, TABLE};
use super::place_links::{record, unrecord};
use crate::linux::file::key;
use crate::linux::file::store::{held, holding};

const SHIPPED: &[u8] = b"/bin/ls /bin/busybox\n/bin/sh /bin/busybox";

fn table() -> Option<Vec<u8>> {
    held(key(TABLE).as_bytes())
}

fn link(path: &[u8], target: &[u8]) -> (Vec<u8>, Vec<u8>) {
    (path.to_vec(), target.to_vec())
}

#[test]
fn a_line_a_package_could_inject_is_skipped() {
    holding(&[(key(TABLE).as_bytes(), SHIPPED)]);
    let added = record(&[
        link(b"/usr/bin/a b", b"x"),
        link(b"/usr/bin/a\n/bin/sh", b"/evil"),
        link(b"/usr/bin/c", b"x\n/bin/sh /evil"),
        link(b"/bin/ls", b"/evil"),
        link(b"/usr/bin/ok", b"jq"),
    ]);
    assert_eq!(added, Some(vec![b"/usr/bin/ok".to_vec()]));
    let want = [SHIPPED, b"\n/usr/bin/ok jq\n"].concat();
    assert_eq!(table(), Some(want), "a table with no last newline gets one first");
}

#[test]
fn a_table_past_its_cap_is_not_written() {
    let full = vec![b'#'; MAX_TABLE as usize - 4];
    holding(&[(key(TABLE).as_bytes(), &full)]);
    assert_eq!(record(&[link(b"/usr/bin/long-name", b"target")]), None);
    assert_eq!(table(), Some(full));
}

#[test]
fn an_uninstall_takes_out_its_own_lines_only() {
    let with = [SHIPPED, b"\n/usr/bin/ok jq\n"].concat();
    holding(&[(key(TABLE).as_bytes(), &with)]);
    assert!(unrecord(&[b"/usr/bin/ok".to_vec()]));
    let left = table().unwrap();
    assert!(left.starts_with(b"/bin/ls /bin/busybox\n/bin/sh /bin/busybox"));
    assert!(!left.windows(10).any(|w| w == b"/usr/bin/o"));
    holding(&[]);
    assert!(unrecord(&[b"/usr/bin/ok".to_vec()]), "no table is no line to take out");
}
