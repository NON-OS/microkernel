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

//! Which member of an image carries the program a market policy names: the
//! first, in image order, with a file whose last copy is an ELF of those
//! bytes, at any path (market/choose.rs, `anywhere`). That member is placed
//! under the listing's own name, held as the listed package is, so the
//! market binds its policy to that program; the rest under their own names.
//! Pure, so the host proofs hold it.

use alloc::collections::BTreeSet;

const ELF: &[u8] = b"\x7fELF";

/// Whether `files`, a member's (name, body) in archive order, carry an ELF
/// whose bytes hash to `want` as the last copy of its name.
pub fn carries(files: &[(&[u8], &[u8])], want: &[u8; 32], hash: impl Fn(&[u8]) -> [u8; 32]) -> bool {
    let mut seen: BTreeSet<&[u8]> = BTreeSet::new();
    files.iter().rev().any(|(n, b)| seen.insert(n) && b.starts_with(ELF) && hash(b) == *want)
}
