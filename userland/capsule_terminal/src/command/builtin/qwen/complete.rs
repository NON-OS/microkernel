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

//! What Tab offers after `qwen`: `window` and every tier; after
//! `qwen window`, every tier.

use alloc::vec::Vec;

use super::tiers::TIERS;
use super::window::WORD;

/// The words that complete `prefix` when `before` is the line up to it,
/// or `None` when `before` is neither `qwen` nor `qwen window`.
pub fn words(before: &[u8], prefix: &[u8]) -> Option<Vec<&'static [u8]>> {
    let typed: Vec<&[u8]> = before.split(|&b| b == b' ').filter(|w| !w.is_empty()).collect();
    let window: &[&'static [u8]] = match typed[..] {
        [b"qwen"] => &[WORD],
        [b"qwen", w] if w == WORD => &[],
        _ => return None,
    };
    let offered = window.iter().chain(TIERS).copied();
    Some(offered.filter(|w| w.starts_with(prefix)).collect())
}
