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
//! What a learn run adds to its report, so it is never taken for an enforced
//! one: a LEARN line under its policy, `enforced=no` on its `[BOX]` line, and
//! at its end the file it learned. Any other box adds nothing.

use alloc::string::String;
use alloc::vec::Vec;

use super::kind::{Kind, ALL};
use super::learned::Learned;
use super::preset::{BoxPolicy, Preset};
use super::tally::Tally;

pub const MODE: &str =
    "mode     LEARN, not enforced: held as offline with no network, to learn what it does";

pub fn mode(b: &BoxPolicy) -> Option<&'static str> {
    (b.preset == Preset::Learn).then_some(MODE)
}

/// The `[BOX]` line's word after the preset.
pub fn enforced(b: &BoxPolicy) -> &'static str {
    if b.preset == Preset::Learn { " enforced=no" } else { "" }
}

/// The learned file, each line indented under its heading.
pub fn learned(b: &BoxPolicy, l: &Learned, t: &Tally) -> Option<String> {
    if b.preset != Preset::Learn {
        return None;
    }
    let mut s = String::from("  learned  the tightest nonos-box 1 file for what it did:\n");
    for line in l.file(&refused(t)).lines() {
        s.push_str(&alloc::format!("    {line}\n"));
    }
    Some(s)
}

/// What it was refused that no file allows: every kind but network, whose
/// destinations become the file's connect lines.
fn refused(t: &Tally) -> String {
    let kinds: Vec<String> = ALL
        .iter()
        .filter(|k| **k != Kind::Network && t.count(**k) > 0)
        .map(|k| alloc::format!("{} {}", k.tag(), t.count(*k)))
        .collect();
    kinds.join(", ")
}
