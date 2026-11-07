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

//! A bare name the terminal's `linux` did not find in its family's tree,
//! looked for along the same PATH in the other families' trees. A Debian
//! family program, as SimpleX's terminal client is, installs into its own
//! tree, which a terminal run reads only when named by its package's word
//! (installed.rs): the person is told where it is and how it is named,
//! rather than that it is not there.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The first of `trees` (each a family's tree in the store) where
/// `holds(tree, path)` finds `program` in one of `dirs`, the PATH's
/// directories, by its place in `trees`; None for a name that is not a
/// plain file name.
pub fn found(
    program: &[u8],
    dirs: &[Vec<u8>],
    trees: &[&[u8]],
    mut holds: impl FnMut(&[u8], &[u8]) -> bool,
) -> Option<usize> {
    if matches!(program, b"" | b"." | b"..") || program.contains(&b'/') {
        return None;
    }
    let absolute = dirs.iter().filter(|d| plain_dir(d));
    trees.iter().position(|tree| absolute.clone().any(|d| holds(tree, &joined(d, program))))
}

/// An absolute directory with no `.` or `..` part, so the path made under
/// a tree stays in it.
fn plain_dir(dir: &[u8]) -> bool {
    dir.first() == Some(&b'/') && !dir.split(|b| *b == b'/').any(|p| p == b"." || p == b"..")
}

/// `program` in the PATH directory `dir`.
fn joined(dir: &[u8], program: &[u8]) -> Vec<u8> {
    let mut path = dir.strip_suffix(b"/").unwrap_or(dir).to_vec();
    path.push(b'/');
    path.extend_from_slice(program);
    path
}

/// What the terminal is told for `shown`, found in `family`'s tree, whose
/// packages' words begin `word` (file/family.rs listed).
pub fn line(shown: &str, family: &str, word: &str) -> String {
    format!(
        "linux: {shown} is installed in the {family} family's tree, not on this PATH; \
         run it by its package's word, `linux {word}<package>` as the Marketplace lists it, \
         or open it from the Marketplace\n"
    )
}
