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
//! A receipt written: the part the challenge covers, then the doc that
//! answers it. The prover asks MkAttestDoc with `challenge(&bound)` between
//! the two calls; nothing else is ever added.

use alloc::vec::Vec;

use super::*;

/// The head and every field before the doc, which the challenge covers.
pub fn bound(
    record: &[u8; RECORD],
    box_word: &[u8],
    policy: &[u8; POLICY],
    launcher: &[u8; LAUNCHER],
) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEAD + 4 * 4 + RECORD + box_word.len() + POLICY + LAUNCHER);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_be_bytes());
    for f in [&record[..], box_word, &policy[..], &launcher[..]] {
        put(&mut out, f);
    }
    out
}

/// The whole receipt: `bound`, the doc asked with its challenge, and the
/// MkAttestEntries entries read with it.
pub fn finish(mut bound: Vec<u8>, doc: &[u8], entries: &[u8], capsules: &[u8]) -> Vec<u8> {
    put(&mut bound, doc);
    put(&mut bound, entries);
    put(&mut bound, capsules);
    bound
}

fn put(out: &mut Vec<u8>, field: &[u8]) {
    out.extend_from_slice(&(field.len() as u32).to_be_bytes());
    out.extend_from_slice(field);
}
