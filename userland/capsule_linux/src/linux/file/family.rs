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

//! Which distribution this process works in. The name the system passes
//! says so, `deb:` or `pacman:` or neither for Alpine, and it is read once
//! at start: each family keeps its own tree, so a Debian `jq` never lands
//! on Alpine's `/usr/bin/jq` and a glibc loader never meets a musl one.

use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Family {
    Alpine = 0,
    Debian = 1,
    Pacman = 2,
}

/// Per family: its tree (Alpine keeps the original `/linux`), and where it
/// records what each package starts as, outside every tree a guest writes.
const PLACES: [(&[u8], &[u8]); 3] = [
    (b"/linux", b"/nonos/linux/apps"),
    (b"/linux-deb", b"/nonos/linux/apps-deb"),
    (b"/linux-pacman", b"/nonos/linux/apps-pacman"),
];

static CHOSEN: AtomicU8 = AtomicU8::new(Family::Alpine as u8);

/// Split `name` into its family and the package, without choosing.
pub fn split(name: &str) -> (Family, &str) {
    match (name.strip_prefix("deb:"), name.strip_prefix("pacman:")) {
        (Some(pkg), _) => (Family::Debian, pkg),
        (_, Some(pkg)) => (Family::Pacman, pkg),
        _ => (Family::Alpine, name),
    }
}

/// Work in `name`'s family from here on, and return the package.
pub fn choose(name: &str) -> &str {
    let (family, pkg) = split(name);
    CHOSEN.store(family as u8, Ordering::Relaxed);
    pkg
}

pub fn chosen() -> Family {
    match CHOSEN.load(Ordering::Relaxed) {
        1 => Family::Debian,
        2 => Family::Pacman,
        _ => Family::Alpine,
    }
}

/// A family's tree and its records.
pub fn places(family: Family) -> (&'static [u8], &'static [u8]) {
    PLACES[family as usize]
}
pub fn root() -> &'static [u8] {
    places(chosen()).0
}
pub fn records() -> &'static [u8] {
    places(chosen()).1
}
