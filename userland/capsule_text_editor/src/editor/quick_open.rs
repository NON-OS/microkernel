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

//! Finding a file by typing part of its name.
//!
//! Opening a file meant knowing its path and typing all of it, or walking the
//! tree with the mouse. In a tree of any size that is the slowest thing in the
//! editor, and it is the reason every editor since has had this.
//!
//! The matching is subsequence, not substring: `mn` finds `main.rs`, and
//! `edmod` finds `editor/mod.rs`. That is what makes it worth typing three
//! letters instead of a path, and it is the whole of the logic, so it lives
//! here free of any editor type and is checked on a host.

/// Whether `needle` appears in `hay` in order, though not necessarily
/// together. Case-insensitive, because nobody reaching for a file by three
/// letters is thinking about capitals.
///
/// An empty needle matches everything, which is what a reader who has just
/// opened the prompt should see.
pub fn subsequence(hay: &[u8], needle: &[u8]) -> bool {
    let mut it = hay.iter();
    needle.iter().all(|n| it.any(|h| h.eq_ignore_ascii_case(n)))
}

/// How well `needle` matches `hay`. Lower is better; `None` is no match.
///
/// Two things decide it, in order. A match inside the file's own name beats
/// one that only lines up across the directories above it, because someone
/// typing `main` wants `src/main.rs` before `main/deep/other.rs`. After that,
/// shorter paths win: the file nearer the root is more often the one meant.
pub fn score(path: &[u8], needle: &[u8]) -> Option<usize> {
    if !subsequence(path, needle) {
        return None;
    }
    let name_at = path.iter().rposition(|&b| b == b'/').map(|p| p + 1).unwrap_or(0);
    let in_name = subsequence(&path[name_at..], needle);
    // The name bonus has to outweigh any plausible path length, or a very
    // short path would beat a real name match.
    let base = if in_name { 0 } else { 1 << 20 };
    Some(base + path.len())
}

/// Index of the best match in `paths`, or `None` when nothing matches.
///
/// Ties keep the earlier entry, so the order the tree reports is what breaks
/// them and the answer does not move between identical searches.
pub fn best(paths: &[&[u8]], needle: &[u8]) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None;
    for (i, p) in paths.iter().enumerate() {
        let Some(s) = score(p, needle) else { continue };
        if best.map(|(bs, _)| s < bs).unwrap_or(true) {
            best = Some((s, i));
        }
    }
    best.map(|(_, i)| i)
}
