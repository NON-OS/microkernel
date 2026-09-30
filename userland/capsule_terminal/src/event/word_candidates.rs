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

use alloc::vec::Vec;

/// The words a command takes in place of a path, completed as the word just
/// after it: `qwen`'s tier. `None` everywhere else.
pub(super) fn word_candidates(before: &[u8], prefix: &[u8]) -> Option<Vec<&'static [u8]>> {
    let mut words = before.split(|&b| b == b' ').filter(|w| !w.is_empty());
    if words.next() != Some(&b"qwen"[..]) || words.next().is_some() {
        return None;
    }
    let tiers = crate::command::builtin::qwen::TIERS.iter().copied();
    Some(tiers.filter(|t| t.starts_with(prefix)).collect())
}
