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

//! Where a vision tier's question ends and its image's name begins.

use alloc::vec::Vec;

use super::is_vision;

/// For a vision tier, a question that ends ` < FILE`, FILE one word naming a
/// PNG or JPEG by its ending, as the question before it and FILE; otherwise
/// the question whole and no file, so "is 2 < 3?" is still a question.
pub fn split<'a>(tier: &[u8], question: &'a [u8]) -> (&'a [u8], Option<&'a [u8]>) {
    if !is_vision(tier) {
        return (question, None);
    }
    let Some(lt) = question.iter().rposition(|&b| b == b'<') else { return (question, None) };
    let path = trim(&question[lt + 1..]);
    let before = &question[..lt];
    let spaced = lt == 0 || before.last().is_some_and(|&b| b == b' ' || b == b'\t');
    let one_word = !path.is_empty() && !path.iter().any(|&b| b == b' ' || b == b'\t');
    if !one_word || !spaced || !image_name(path) {
        return (question, None);
    }
    (trim(before), Some(path))
}

/// Whether `path` ends .png, .jpg or .jpeg, in any case.
fn image_name(path: &[u8]) -> bool {
    let low: Vec<u8> = path.iter().map(u8::to_ascii_lowercase).collect();
    [&b".png"[..], b".jpg", b".jpeg"].iter().any(|e| low.len() > e.len() && low.ends_with(e))
}

fn trim(b: &[u8]) -> &[u8] {
    let start = b.iter().position(|&c| c != b' ' && c != b'\t').unwrap_or(b.len());
    let end = b.iter().rposition(|&c| c != b' ' && c != b'\t').map_or(start, |i| i + 1);
    &b[start..end]
}
