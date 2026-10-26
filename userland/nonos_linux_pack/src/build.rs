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

//! Writing a pack: the one layout `Pack::parse` accepts, from the files of
//! a package the installer has verified. A source it could not write so
//! that the parser takes it back is refused here, before anything lands.

use alloc::vec::Vec;

use crate::format::{up, DATA_ALIGN, ENTRY_LEN, FILE_ALIGN, HEADER_LEN};
use crate::format::{MAX_ENTRIES, MAX_PACK, TARGET_MAX};
use crate::kind::Kind;
use crate::refused::Refused;

/// One file, directory or link of a package: its path under the package's
/// root, and its bytes, or a link's target, borrowed: a package is not
/// copied once more to be packed.
pub struct Source<'a> {
    pub name: Vec<u8>,
    pub kind: Kind,
    pub exec: bool,
    pub bytes: &'a [u8],
}

pub fn build(mut sources: Vec<Source>) -> Result<Vec<u8>, Refused> {
    sources.sort_by(|a, b| a.name.cmp(&b.name));
    let ok = |s: &Source| match s.kind {
        Kind::Dir => s.bytes.is_empty(),
        Kind::Link => (1..=TARGET_MAX).contains(&s.bytes.len()) && !s.bytes.contains(&0),
        Kind::File => true,
    };
    let fits = sources.len() <= MAX_ENTRIES
        && sources.iter().all(|s| crate::name::valid(&s.name) && ok(s))
        && sources.windows(2).all(|w| w[0].name < w[1].name);
    if !fits {
        return Err(Refused::Build);
    }
    let names_off = HEADER_LEN + sources.len() * ENTRY_LEN;
    let names_len: usize = sources.iter().map(|s| s.name.len()).sum();
    let data_off = up(names_off + names_len, DATA_ALIGN).ok_or(Refused::Build)?;
    let mut at = Vec::with_capacity(sources.len());
    let mut end = data_off;
    for s in &sources {
        let start = match s.kind {
            Kind::Dir => 0,
            _ if s.bytes.is_empty() => data_off,
            _ => up(end, FILE_ALIGN).ok_or(Refused::Build)?,
        };
        end = end.max(start + s.bytes.len());
        at.push(start);
    }
    if end > MAX_PACK {
        return Err(Refused::Build);
    }
    crate::write::write(&sources, &at, (names_off, names_len, data_off), end)
}
