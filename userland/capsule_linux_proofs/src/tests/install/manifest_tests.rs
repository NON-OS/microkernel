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

//! What a package wrote, kept so an uninstall takes out that and nothing
//! else: the record reads back what was written, a file another package
//! also wrote stays, and only the package's own link lines leave the table.

use crate::install::manifest::{decode, drop_links, encode, only_mine, Manifest};

fn m(files: &[&[u8]], links: &[&[u8]]) -> Manifest {
    Manifest {
        files: files.iter().map(|p| p.to_vec()).collect(),
        links: links.iter().map(|p| p.to_vec()).collect(),
    }
}

#[test]
fn a_manifest_reads_back_what_was_written() {
    let kept = m(&[b"/usr/bin/jq", b"/usr/lib/lib jq.so"], &[b"/usr/bin/jq-alias"]);
    assert_eq!(decode(&encode(&kept)), kept);
    assert_eq!(decode(&encode(&Manifest::default())), Manifest::default());
}

#[test]
fn a_path_with_a_newline_is_never_written_or_misread() {
    let kept = m(&[b"/a\nl /etc/passwd", b"/b"], &[]);
    let back = decode(&encode(&kept));
    assert_eq!(back, m(&[b"/b"], &[]));
}

#[test]
fn malformed_lines_are_skipped_not_guessed_at() {
    let back = decode(b"f /a\nx /b\nf \nl\nl /c\n\nf/d\n");
    assert_eq!(back, m(&[b"/a"], &[b"/c"]));
}

#[test]
fn a_file_another_package_wrote_stays() {
    let mine = m(&[b"/usr/bin/jq", b"/usr/lib/libonig.so"], &[]);
    let other = m(&[b"/usr/lib/libonig.so"], &[]);
    let go = only_mine(&mine.files, &[other], |x| &x.files);
    assert_eq!(go, vec![b"/usr/bin/jq".to_vec()]);
    let alone = only_mine(&mine.files, &[], |x| &x.files);
    assert_eq!(alone, mine.files);
}

#[test]
fn only_the_packages_own_link_lines_leave_the_table() {
    let table = b"/bin/sh /bin/busybox\n/usr/bin/jq-alias jq\n/lib usr/lib\n";
    let (left, dropped) = drop_links(table, &[b"/usr/bin/jq-alias".to_vec()]);
    assert_eq!(dropped, 1);
    assert_eq!(left, b"/bin/sh /bin/busybox\n/lib usr/lib\n".to_vec());
    // A path that is only the start of another line's path takes nothing.
    let (same, none) = drop_links(table, &[b"/bin".to_vec()]);
    assert_eq!(none, 0);
    assert_eq!(same, table.to_vec());
}
