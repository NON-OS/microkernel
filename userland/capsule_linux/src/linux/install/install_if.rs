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

//! Alpine's install_if: a package that comes in by itself once every name
//! on its `i:` line is installed. Alpine writes these for the parts of a
//! program a desktop needs beside it, and on this one the rules that matter
//! name a Wayland display: qt6-qtwayland, Qt's Wayland platform, comes with
//! qt6-qtbase-x11 where wayland-libs-server, a compositor, is installed.
//! Here the compositor is the system's own (the personality serves the
//! display), so that name counts as installed. Without it a Qt program
//! found no Wayland platform and could not open a window.
//!
//! A rule must name at least one package this install brought: the system's
//! name alone brings nothing. What comes in is held to the signed index and
//! its checksums like every other package of the closure.

use alloc::vec::Vec;

use super::index::{Index, Pkg};

/// The names this system provides without a package.
pub const SYSTEM: &[&str] = &["wayland-libs-server"];

/// The packages whose rules `done` (the names installed so far) meets, not
/// installed yet, in index order.
pub fn triggered<'a>(index: &'a Index, done: &[alloc::string::String]) -> Vec<&'a Pkg> {
    let have = |n: &str| done.iter().any(|d| d == n);
    let met = |p: &Pkg| {
        !p.install_if.is_empty()
            && p.install_if.iter().all(|n| have(n) || SYSTEM.contains(&n.as_str()))
            && p.install_if.iter().any(|n| have(n))
    };
    index.pkgs.iter().filter(|p| !have(&p.name) && met(p)).collect()
}

/// The name of the first package a rule brings, or None when none is due.
pub fn next(index: &Index, done: &[alloc::string::String]) -> Option<alloc::string::String> {
    triggered(index, done).into_iter().next().map(|p| p.name.clone())
}
