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

//! An entry document of exactly the length asked, built with the abi's own
//! encoder: enough releases, then their notes and the description to fit.

use nonos_marketplace_abi::encode_entry_document;

use crate::entry::entry;
use crate::release::release;

/// The most a description or a release's note holds.
const TEXT: usize = 4096;

pub fn document_of(len: usize) -> Vec<u8> {
    let mut e = entry();
    let base = encode_entry_document(&e).len();
    e.releases.push(release(0));
    let each = encode_entry_document(&e).len() - base;
    let k = (0..).find(|k| base + k * each + (k + 1) * TEXT >= len).unwrap_or(0);
    e.releases = (0..k).map(release).collect();
    let mut need = len - base - k * each;
    let mut text = || {
        let n = need.min(TEXT);
        need -= n;
        "n".repeat(n)
    };
    e.description = text();
    for r in &mut e.releases {
        r.validation.note = text();
    }
    let doc = encode_entry_document(&e);
    assert_eq!(doc.len(), len, "no document of {len} bytes from this base");
    doc
}
