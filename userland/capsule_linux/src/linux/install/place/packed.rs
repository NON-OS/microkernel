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
//! Putting a package's files into the store, under the Linux root.
//! A package's files as one pack (nonos_linux_pack; the design note is
//! docs/design/linux-package-pack.md): every file it may write and the
//! trailer this machine mints beside each ELF, one store object however
//! many files. A market program held until its policy is bound stays a file
//! of its own (minted.rs): its binding writes beside it later, and a pack
//! does not change once it is in.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use nonos_linux_pack::Source;

use super::super::market::minted::{beside, TRAILER};
use super::super::tar::{Entry, Kind};
use super::super::Why;
use super::entry::{allowed, one, Placed};
use super::source::{elf, src, vouched};
use crate::linux::file::visible;

/// What of a package went in: every file's path, and how many of its held
/// programs the store would not take.
pub(super) struct Put {
    pub landed: Vec<Vec<u8>>,
    pub failed: usize,
}

/// The package's files in as `name`'s pack; then its held programs, each
/// as a file. A member named twice is the later one, as a tar is read.
pub(super) fn place(entries: &[Entry], name: &str, held: bool) -> Result<Put, Why> {
    let mut files: BTreeMap<Vec<u8>, &Entry> = BTreeMap::new();
    let mut loose = Vec::new();
    for e in entries.iter().filter(|e| matches!(e.kind, Kind::File)) {
        match held && elf(e) {
            true => loose.push(e),
            false if allowed(&e.name) => drop(files.insert(visible(b"/", &e.name), e)),
            false => {}
        }
    }
    let minted = |(at, e): (&Vec<u8>, &&Entry)| Some((beside(at, TRAILER), vouched(&e.body)?));
    let trailers: Vec<(Vec<u8>, Vec<u8>)> =
        files.iter().filter(|(_, e)| elf(e)).filter_map(minted).collect();
    let mut sources: Vec<Source> = files.iter().map(|(at, e)| src(at, &e.body)).collect();
    sources.extend(trailers.iter().map(|(at, t)| src(at, t)));
    super::land::land(name, sources)?;
    let mut put = Put { landed: files.into_keys().collect(), failed: 0 };
    for e in loose {
        match one(e, true) {
            Placed::Landed(at) => put.landed.push(at),
            Placed::Failed => put.failed += 1,
            Placed::Refused => {}
        }
    }
    Ok(put)
}
