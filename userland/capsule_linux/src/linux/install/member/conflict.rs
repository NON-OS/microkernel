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

//! A path another installed package already holds, as apk and dpkg refuse
//! one: otherwise a second package would silently take the first one's
//! file, and removing the second would take it out from under the first.
//! Links already keep their first owner (place_links.rs); this holds files
//! to the same rule. Pure, so the host proofs hold it.

use alloc::vec::Vec;

use super::super::manifest::Manifest;

/// The first of `mine` that one of `others` holds as a file or a link.
pub fn held_by_another<'a>(mine: &'a [Vec<u8>], others: &[Manifest]) -> Option<&'a [u8]> {
    mine.iter()
        .find(|p| others.iter().any(|m| m.files.contains(p) || m.links.contains(p)))
        .map(Vec::as_slice)
}
