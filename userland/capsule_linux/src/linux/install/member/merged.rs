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

//! Debian's merged /usr, laid without reaching the image's own files. Pure,
//! so the host proofs hold it.
//!
//! A link at a whole directory (/bin -> /usr/bin) is laid only where nothing
//! the image carries, and no program the built-in BusyBox answers for, is
//! under it: /bin's would make a package's /usr/bin/sh every guest's /bin/sh,
//! and send /bin/qwenchat to a file that is not there. Elsewhere a package
//! gets one link per program it places, /bin/x -> /usr/bin/x, by the same rule.

use alloc::vec::Vec;

use super::image::shadows;

/// Each directory a Debian tree has as a link, and where it leads.
pub const MERGED: [(&[u8], &[u8]); 4] = [
    (b"/bin", b"/usr/bin"),
    (b"/sbin", b"/usr/sbin"),
    (b"/lib", b"/usr/lib"),
    (b"/lib64", b"/usr/lib64"),
];

/// A link (path, target).
pub type Link = (Vec<u8>, Vec<u8>);

/// Whether something holds at a path: the built-in, or another package.
pub type Holds<'a> = &'a dyn Fn(&[u8]) -> bool;

/// Whether a link may be laid at `at`: no file of the `image` is at it or
/// under it, and the built-in answers for nothing there (`built_in`).
pub fn may_link(at: &[u8], image: &[&[u8]], built_in: Holds<'_>) -> bool {
    !shadows(at, true, image) && !built_in(at)
}

/// The directory links that may be laid whole.
pub fn whole(image: &[&[u8]], built_in: Holds<'_>) -> Vec<(&'static [u8], &'static [u8])> {
    MERGED.iter().copied().filter(|(from, _)| may_link(from, image, built_in)).collect()
}

/// For a package placing `placed`, the links by name it gets where its
/// directory's may not be laid whole, past any path another package holds
/// (`held`).
pub fn by_name(placed: &[Vec<u8>], image: &[&[u8]], built_in: Holds, held: Holds) -> Vec<Link> {
    let mut out: Vec<Link> = Vec::new();
    for (dir, to) in MERGED.iter().filter(|(d, _)| !may_link(d, image, built_in)) {
        for p in placed {
            let Some(name) = p.strip_prefix(*to).and_then(|r| r.strip_prefix(b"/")) else {
                continue;
            };
            let from = [*dir, b"/", name].concat();
            let fresh = !out.iter().any(|(f, _)| *f == from);
            let single = !name.is_empty() && !name.contains(&b'/');
            if single && fresh && !held(&from) && may_link(&from, image, built_in) {
                out.push((from, p.clone()));
            }
        }
    }
    out
}
