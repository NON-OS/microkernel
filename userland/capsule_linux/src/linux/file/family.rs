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

use alloc::format;
use alloc::string::String;
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

/// Each family's prefix on a run's name, and the namespace its market
/// listings are under (nonos-market-catalogue NAMESPACES): Debian's are
/// `linux.kali.<package>`, pacman's `linux.blackarch.<package>`, Alpine's
/// `linux.<package>`.
const WORDS: [(&str, &str); 2] = [("deb:", "kali."), ("pacman:", "blackarch.")];

/// The longest word taken, as the desktop shell's package name.
const WORD_MAX: usize = 64;

/// A run's `name` as its listing names it, `deb:simplex` as `kali.simplex`:
/// the word the desktop shell writes into the Terminal's `linux` line, as
/// it takes a package name only, with no `:` in it.
pub fn listed(name: &str) -> String {
    match WORDS.iter().find_map(|(run, word)| Some((word, name.strip_prefix(run)?))) {
        Some((word, pkg)) => format!("{word}{pkg}"),
        None => String::from(name),
    }
}

/// The run name `word`, as the Terminal's `linux` command was given it,
/// names: `kali.simplex` is `deb:simplex`, a word in no family's namespace
/// Alpine's own. None for a word that is no package name (a path, an
/// option, a `:`), which the Terminal looks for as a program alone.
pub fn run_name(word: &str) -> Option<String> {
    let b = word.as_bytes();
    let first = b.first().is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    let rest =
        b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"._+-".contains(c));
    if !first || !rest || b.len() > WORD_MAX {
        return None;
    }
    match WORDS.iter().find_map(|(run, word_of)| Some((run, word.strip_prefix(word_of)?))) {
        Some((_, "")) => None,
        Some((run, pkg)) => Some(format!("{run}{pkg}")),
        None => Some(String::from(word)),
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
