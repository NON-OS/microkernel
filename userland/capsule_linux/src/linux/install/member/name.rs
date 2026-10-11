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

//! The member names a package may not place, which fail the whole package
//! rather than land somewhere else. Pure, so the host proofs hold it.
//!
//! A name with a `..` component, or one that is absolute once ustar's
//! prefix is applied, is malformed: resolving it under the Linux root would
//! keep it inside, but would quietly send a write elsewhere, as
//! `x/../../etc/nonos-links` onto the link table. And a few paths under the
//! root are the personality's own, which no package writes.

use alloc::vec::Vec;

/// Files under the root the personality keeps, reads or writes itself.
pub const RESERVED_FILES: [&[u8]; 3] =
    [b"/etc/nonos-links", b"/etc/nonos-boot-guest", b"/etc/nonos-install-max"];
/// Directories under the root that are the personality's (its keymap).
pub const RESERVED_DIRS: [&[u8]; 1] = [b"/usr/share/nonos"];

/// Why a member name fails its package.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    /// It names a `..` component.
    Escapes,
    /// It begins at `/`.
    Absolute,
    /// It names one of the personality's own paths.
    Reserved,
}

impl Refused {
    /// The reason, as the install's log line gives it.
    pub fn about(self) -> &'static str {
        match self {
            Refused::Escapes => "it names a `..` component",
            Refused::Absolute => "it is absolute",
            Refused::Reserved => "it is one of the personality's own paths",
        }
    }
}

pub fn member_refused(name: &[u8]) -> Option<Refused> {
    if name.first() == Some(&b'/') {
        return Some(Refused::Absolute);
    }
    let parts: Vec<&[u8]> =
        name.split(|b| *b == b'/').filter(|p| !p.is_empty() && *p != b".").collect();
    if parts.iter().any(|p| *p == b"..") {
        return Some(Refused::Escapes);
    }
    let at: Vec<u8> = parts.iter().flat_map(|p| [&b"/"[..], p].concat()).collect();
    let under = |dir: &[u8]| at == dir || (at.starts_with(dir) && at.get(dir.len()) == Some(&b'/'));
    let reserved =
        RESERVED_FILES.contains(&at.as_slice()) || RESERVED_DIRS.iter().any(|d| under(d));
    reserved.then_some(Refused::Reserved)
}
