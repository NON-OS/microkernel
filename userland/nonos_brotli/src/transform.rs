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

use crate::ferment::ferment;
use crate::transforms::{AFFIXES, AFFIX_AT, TRIPLES};

const _: () = assert!(AFFIXES.len() == 217 && TRIPLES.len() == 3 * crate::dict::TRANSFORMS);

/// Room for any transformed word: 24 bytes of word and 13 of affixes.
pub(crate) type Word = [u8; 40];

/// Apply transform `id` to `word` into `dst`, returning the length.
pub(crate) fn apply(word: &[u8], id: usize, dst: &mut Word) -> usize {
    let (kind, n) = (TRIPLES[3 * id + 1] as usize, word.len());
    let mut at = put(dst, 0, affix(TRIPLES[3 * id]));
    let body = match kind {
        1..=9 => &word[..n.saturating_sub(kind)],
        12..=20 => &word[(kind - 11).min(n)..],
        _ => word,
    };
    let end = put(dst, at, body);
    match kind {
        10 if end > at => {
            ferment(&mut dst[at..end], 0);
        }
        11 => {
            while at < end {
                at += ferment(&mut dst[..end], at);
            }
        }
        _ => {}
    }
    put(dst, end, affix(TRIPLES[3 * id + 2]))
}

fn put(dst: &mut Word, at: usize, text: &[u8]) -> usize {
    dst[at..at + text.len()].copy_from_slice(text);
    at + text.len()
}

fn affix(i: u8) -> &'static [u8] {
    let at = AFFIX_AT[i as usize] as usize;
    let all = AFFIXES.as_bytes();
    &all[at + 1..at + 1 + all[at] as usize]
}
