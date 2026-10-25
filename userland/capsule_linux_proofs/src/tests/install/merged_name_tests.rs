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

//! C10e: the links by name a Debian package gets, and the ones it does not:
//! at an image file, at a program the built-in answers for, at a path held
//! elsewhere, twice, or for anything but a program directly in /usr/bin or
//! /usr/sbin.

use crate::install::member::merged::{by_name, may_link, whole};

const IMAGE: [&[u8]; 2] = [b"/bin/qwenchat", b"/etc/ssl/cert.pem"];

/// A built-in that answers for /bin/sh alone.
fn sh_only(p: &[u8]) -> bool {
    p == b"/bin/sh"
}

fn links(placed: &[&str], held: &dyn Fn(&[u8]) -> bool) -> Vec<(String, String)> {
    let placed: Vec<Vec<u8>> = placed.iter().map(|p| p.as_bytes().to_vec()).collect();
    let text = |b: &[u8]| String::from_utf8(b.to_vec()).unwrap();
    by_name(&placed, &IMAGE, &sh_only, held).iter().map(|(f, t)| (text(f), text(t))).collect()
}

#[test]
fn a_directory_link_is_laid_whole_only_over_nothing_of_the_image_s() {
    let laid: Vec<&[u8]> = whole(&IMAGE, &sh_only).iter().map(|l| l.0).collect();
    assert_eq!(laid, [&b"/sbin"[..], b"/lib", b"/lib64"], "/bin holds qwenchat");
    assert!(!may_link(b"/etc", &IMAGE, &|_| false) && !may_link(b"/", &IMAGE, &|_| false));
    assert!(may_link(b"/etc/ssl/certs", &IMAGE, &|_| false));
}

#[test]
fn a_program_in_usr_bin_gets_its_bin_name_unless_something_holds_it() {
    let nobody = |_: &[u8]| false;
    let got = links(&["/usr/bin/jq", "/usr/bin/sh", "/usr/bin/qwenchat", "/usr/bin/jq"], &nobody);
    assert_eq!(got, [("/bin/jq".into(), "/usr/bin/jq".into())]);
    let held = |p: &[u8]| p == b"/bin/jq";
    assert!(links(&["/usr/bin/jq"], &held).is_empty(), "another package's, or its own");
    let odd = ["/usr/bin", "/usr/bin/", "/usr/bin/a/b", "/usr/binx/y", "/usr/sbin/z", "/bin/c"];
    assert!(links(&odd, &nobody).is_empty(), "/sbin is laid whole here, so z needs none");
}
