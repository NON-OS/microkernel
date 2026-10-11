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

//! C10e: Debian's merged /usr, laid against the image this tree builds and
//! the built-in BusyBox it embeds, then read back as a guest's walk reads
//! it. /bin/qwenchat stays the image's file, and a package's /usr/bin/sh
//! never becomes /bin/sh.

use super::image_files;
use crate::applets::has;
use crate::install::member::merged::{by_name, whole, Link, MERGED};
use crate::install::place_links::{record, unrecord};
use crate::linux::file::key;
use crate::linux::file::store::holding;
use crate::linux::guest::links::Links;

const STORE: &str = include_str!("../../../../../tools/nix/store.json");
static BUSYBOX: &[u8] = include_bytes!("../../../../capsule_linux/guests/busybox.elf");
const BIN_DIRS: [&[u8]; 4] = [b"/bin", b"/sbin", b"/usr/bin", b"/usr/sbin"];

/// built_in.rs's serves, at a path or for a directory of programs.
fn built_in(p: &[u8]) -> bool {
    let serves = |p: &[u8]| {
        let at = p.iter().rposition(|b| *b == b'/').unwrap_or(0);
        let name = &p[at + 1..];
        BIN_DIRS.contains(&&p[..at]) && (name == b"busybox" || has(BUSYBOX, name))
    };
    serves(p) || serves(&[p, b"/busybox"].concat())
}

/// lay_out's two steps, then a Debian package placing `placed`, split at
/// spaces; the table as a guest loads it.
fn install(placed: &str) -> Links {
    let files = image_files::image_files(STORE).unwrap();
    let image: Vec<&[u8]> = files.iter().map(|f| f.as_bytes()).collect();
    let laid = whole(&image, &built_in);
    let refused: Vec<Vec<u8>> =
        MERGED.iter().filter(|m| !laid.contains(m)).map(|m| m.0.to_vec()).collect();
    assert!(unrecord(&refused));
    let dirs: Vec<Link> = laid.iter().map(|(f, t)| (f.to_vec(), t.to_vec())).collect();
    assert!(record(&dirs).is_some());
    let placed: Vec<Vec<u8>> = placed.split(' ').map(|p| p.as_bytes().to_vec()).collect();
    assert!(record(&by_name(&placed, &image, &built_in, &|_| false)).is_some());
    Links::try_load().unwrap()
}

#[test]
fn bin_qwenchat_is_still_the_image_s_after_a_debian_install() {
    // An install before C10e left a whole /bin link; it is taken out.
    holding(&[(key(b"/etc/nonos-links").as_bytes(), b"/bin usr/bin\n/lib usr/lib\n")]);
    let links =
        install("/usr/bin/sh /usr/bin/qwenchat /usr/bin/bash /usr/sbin/iptables /usr/sbin/route");
    let walk = |p: &[u8]| links.follow(p.to_vec(), true);
    for kept in [&b"/bin/qwenchat"[..], b"/bin/qwenchat-x86_64", b"/bin/sh", b"/sbin/route"] {
        assert_eq!(walk(kept), kept, "{:?}", core::str::from_utf8(kept));
    }
    assert_eq!(walk(b"/bin/bash"), b"/usr/bin/bash", "BusyBox has no bash");
    assert_eq!(walk(b"/sbin/iptables"), b"/usr/sbin/iptables", "nor iptables");
    assert_eq!(walk(b"/lib64/ld-linux-x86-64.so.2"), b"/usr/lib64/ld-linux-x86-64.so.2");
}
