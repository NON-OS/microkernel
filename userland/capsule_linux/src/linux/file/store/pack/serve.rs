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

//! Serving a guest-visible path out of the packs: whether it is there, and
//! its bytes. A path the packs do not hold is None, and
//! the store's own tree answers it instead.

use alloc::vec::Vec;

use nonos_linux_pack::Kind;

use super::table::with;

/// A visible path as a pack names it: no leading `/`.
pub(super) fn name(visible: &[u8]) -> &[u8] {
    visible.strip_prefix(b"/").unwrap_or(visible)
}

/// Its size and whether it is a directory: one a pack names, or one with a
/// pack's files below it.
pub fn stat(visible: &[u8]) -> Option<(u64, bool)> {
    let n = name(visible);
    with(|packs| {
        packs.iter().find_map(|p| match p.index.find(n) {
            Some(e) => Some((e.data.len() as u64, e.kind == Kind::Dir)),
            None if !n.is_empty() && !p.index.under(n).is_empty() => Some((0, true)),
            None => None,
        })
    })
}

/// Its bytes, up to `max`, read whole and served only once they are its
/// digest: a cut read is never checked, so it is never served.
pub fn read(visible: &[u8], max: u32) -> Option<Result<Vec<u8>, &'static str>> {
    let n = name(visible);
    with(|packs| {
        let p = packs.iter_mut().find(|p| p.index.find(n).is_some_and(|e| e.kind == Kind::File))?;
        let e = p.index.find(n)?;
        let read = p.stream.read_window(e.data.start as u64, u32::try_from(e.data.len()).ok()?);
        Some(read.and_then(|mut bytes| match p.index.verify(&e, &bytes).is_ok() {
            true => Ok({
                bytes.truncate(max as usize);
                bytes
            }),
            false => Err("a packed file is not its digest"),
        }))
    })
}
