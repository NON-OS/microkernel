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

/* The rule `resolve` is, over a table of weights and a proof mark, so a
 * proof can hold it to tables the shipped ones do not reach. */

use super::super::need::{tier_fits, Room, STICK_TIER};
use super::Source;

pub fn resolve_in(
    weights: &'static [(&'static str, u64)],
    proven: impl Fn(&str) -> bool,
    chosen: &[u8],
    memory: Option<u64>,
    room: Room,
) -> (&'static str, Source) {
    let word_byte =
        |c: &u8| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'.' || *c == b'-';
    let word = &chosen[..chosen.iter().position(|c| !word_byte(c)).unwrap_or(chosen.len())];
    if let Some(&(t, _)) = weights.iter().find(|(t, _)| t.as_bytes() == word && !word.is_empty()) {
        return (t, Source::Chosen);
    }
    let fits = |(t, b): &(&str, u64)| memory.and_then(|m| tier_fits(room, t, *b, m));
    if weights.iter().any(|w| w.0 == STICK_TIER && proven(w.0) && fits(w) != Some(false)) {
        return (STICK_TIER, Source::Stick);
    }
    match weights.iter().rev().find(|w| proven(w.0) && fits(w) == Some(true)) {
        Some(&(t, _)) => (t, Source::Largest),
        None => {
            (weights.iter().find(|w| proven(w.0)).map_or(STICK_TIER, |w| w.0), Source::Smallest)
        }
    }
}
