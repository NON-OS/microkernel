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

//! A package's links join the table the personality follows: added beside
//! the lines already there, started when there is no table, and never
//! written over a table the store did not answer for.

use super::place_links::record;
use crate::linux::file::key;
use crate::linux::file::store::{held, holding, refusing};

const TABLE: &[u8] = b"/etc/nonos-links";
const SHIPPED: &[u8] = b"/bin/ls /bin/busybox\n/bin/sh /bin/busybox\n";

fn table() -> Option<Vec<u8>> {
    held(key(TABLE).as_bytes())
}

fn link(path: &[u8], target: &[u8]) -> (Vec<u8>, Vec<u8>) {
    (path.to_vec(), target.to_vec())
}

#[test]
fn a_package_s_links_join_the_lines_already_there() {
    let at = key(TABLE);
    holding(&[(at.as_bytes(), SHIPPED)]);
    let added = record(&[link(b"/usr/bin/jq-alias", b"jq")]);
    assert_eq!(added, Some(vec![b"/usr/bin/jq-alias".to_vec()]));
    let mut want = SHIPPED.to_vec();
    want.extend_from_slice(b"/usr/bin/jq-alias jq\n");
    assert_eq!(table(), Some(want));
}

#[test]
fn with_no_table_yet_the_package_s_lines_start_one() {
    holding(&[]);
    assert_eq!(
        record(&[link(b"/usr/bin/jq-alias", b"jq")]),
        Some(vec![b"/usr/bin/jq-alias".to_vec()])
    );
    assert_eq!(table(), Some(b"/usr/bin/jq-alias jq\n".to_vec()));
}

#[test]
fn a_table_the_store_did_not_answer_for_is_left_as_it_is() {
    let at = key(TABLE);
    for why in ["vfs ipc failed", "vfs open failed", "vfs read failed"] {
        holding(&[(at.as_bytes(), SHIPPED)]);
        refusing(why);
        assert_eq!(record(&[link(b"/usr/bin/jq-alias", b"jq")]), None, "{why}");
        assert_eq!(table(), Some(SHIPPED.to_vec()), "{why}: BusyBox's links were written over");
    }
}

#[test]
fn a_package_with_no_links_never_reads_the_table() {
    holding(&[]);
    refusing("vfs ipc failed");
    assert_eq!(record(&[]), Some(Vec::new()));
}
