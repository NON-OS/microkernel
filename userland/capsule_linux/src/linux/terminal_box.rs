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

//! `linux --box <box> <program>`: the box armed before the program is named.

use super::say::say;

const USAGE: &[u8] =
    b"usage: linux [--box strict|offline|allow:<folder>,...] <program> [arguments]\n";

/// The program the words name, after a box if one is named first; None when
/// there is none to run, with why said where the person who typed it reads it.
pub(super) fn program<'a>(parts: &mut impl Iterator<Item = &'a [u8]>) -> Option<&'a [u8]> {
    let mut first = parts.next().filter(|p| !p.is_empty());
    // The terminal names the report's key ahead of every boxed run's words.
    let key = first.and_then(crate::linux::boxed::key_of);
    if key.is_some() {
        first = parts.next().filter(|p| !p.is_empty());
    }
    if first == Some(b"--box".as_slice()) {
        match parts.next().map(crate::linux::boxed::for_terminal) {
            Some(Ok(policy)) => crate::linux::boxed::arm(policy, key),
            Some(Err(why)) => {
                say(alloc::format!("linux: --box refused: {why}\n").as_bytes());
                return None;
            }
            None => {}
        }
        first = parts.next().filter(|p| !p.is_empty());
        super::selftest_panic::maybe(first);
    }
    if first.is_none() {
        say(USAGE);
    }
    first
}
