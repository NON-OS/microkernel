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

//! A file's bytes served only when they are its digest. Every file a run
//! serves, an exec's included, comes through here.

use crate::entry::Entry;
use crate::format::{DIGEST_LEN, TARGET_MAX};
use crate::index::Index;
use crate::kind::Kind;
use crate::refused::Refused;

impl<'a> Index<'a> {
    /// `bytes`, read from the pack at `e.data`, once they are `e`'s: its
    /// length, its digest, and for a link a target with no NUL. Every
    /// file served, an exec's included, comes through here.
    pub fn verify<'b>(&self, e: &Entry<'a>, bytes: &'b [u8]) -> Result<&'b [u8], Refused> {
        let hash = blake3::hash(bytes);
        let link_ok = e.kind != Kind::Link || (bytes.len() <= TARGET_MAX && !bytes.contains(&0));
        match e.kind != Kind::Dir
            && bytes.len() == e.data.len()
            && link_ok
            && hash.as_bytes()[..DIGEST_LEN] == e.digest
        {
            true => Ok(bytes),
            false => Err(Refused::Digest),
        }
    }
}
