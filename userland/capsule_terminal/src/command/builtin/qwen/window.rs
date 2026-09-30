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

//! `qwen window [tier]`: the tier in its own desktop window rather than in
//! this tab. Only as the first word after `qwen`; `qwen small window ...`
//! still asks a question that starts with "window".

use alloc::vec::Vec;

use super::ask::Ask;
use super::tiers::TIERS;
use crate::term::util::is_space;

/// The word that asks for a window, as typed and as `tool.qwen` reads it.
pub const WORD: &[u8] = b"window";

#[derive(Debug, PartialEq, Eq)]
pub enum Window<'a> {
    /// Open this tier: the one typed, or the first with none typed.
    Open(&'static [u8]),
    /// What followed `window` names no tier; nothing is started.
    NotATier(&'a [u8]),
}

/// `ask` as a window request, or `None` when it is not one: a tier came
/// first, or the first word is not `window` (`windows` is not).
pub fn parse<'a>(ask: &Ask<'a>) -> Option<Window<'a>> {
    if ask.tier.is_some() {
        return None;
    }
    let rest = ask.question.strip_prefix(WORD)?;
    if rest.first().is_some_and(|&b| !is_space(b)) {
        return None;
    }
    let start = rest.iter().position(|&b| !is_space(b)).unwrap_or(rest.len());
    let word = &rest[start..];
    if word.is_empty() {
        return Some(Window::Open(TIERS[0]));
    }
    Some(TIERS.iter().copied().find(|t| *t == word).map_or(Window::NotATier(word), Window::Open))
}

/// The argument `tool.qwen` takes for a window of `tier`: the word, a NUL,
/// and the tier word.
pub fn request(tier: &[u8]) -> Vec<u8> {
    let mut argv = Vec::with_capacity(WORD.len() + 1 + tier.len());
    argv.extend_from_slice(WORD);
    argv.push(0);
    argv.extend_from_slice(tier);
    argv
}
