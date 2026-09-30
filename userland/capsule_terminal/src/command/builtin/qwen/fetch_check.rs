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

/*
 * The words of `qwen get` and `qwen tiers` checked before the fetcher
 * starts, and the line history keeps for them: the checked words alone, or
 * only `qwen` for a line refused, since the rest may have been a question.
 */

use alloc::vec::Vec;

use super::fetch_words::{request, Fetch};
use super::tiers::TIERS;

/* Why `fetch` is refused before the fetcher starts, or `None` to start it. */
pub fn refusal(fetch: &Fetch<'_>) -> Option<Vec<u8>> {
    let Fetch::Get(tiers) = fetch else { return None };
    if tiers.is_empty() {
        return Some(b"qwen get: name one or more tiers; `qwen tiers` lists them".to_vec());
    }
    let word = tiers.iter().find(|w| !TIERS.contains(w))?;
    Some([&b"qwen get: "[..], word, b" is not a tier; `qwen tiers` lists them"].concat())
}

/* The line history keeps for `fetch`. */
pub fn kept(fetch: &Fetch<'_>) -> Vec<u8> {
    if refusal(fetch).is_some() {
        return b"qwen".to_vec();
    }
    let words = request(fetch).into_iter().map(|b| if b == 0 { b' ' } else { b });
    b"qwen ".iter().copied().chain(words).collect()
}
