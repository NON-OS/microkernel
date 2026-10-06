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
 * The tiers the Linux personality pins, smallest first, each with the
 * summed size of its files. build.rs writes the table from
 * capsule_linux/src/linux/file/models/pinned_*.rs, and the check below
 * holds it to labels.rs, so neither can drift from the pins unseen.
 */

use super::labels::LABELS;

include!(concat!(env!("OUT_DIR"), "/qwen_pins.rs"));

const fn same(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn named(tier: &[u8]) -> bool {
    let mut i = 0;
    while i < LABELS.len() {
        if same(LABELS[i].0, tier) {
            return true;
        }
        i += 1;
    }
    false
}

/* Every pinned tier is named, and nothing else is: the counts match too. */
const fn agree() -> bool {
    let mut i = 0;
    while i < PINNED.len() {
        if !named(PINNED[i].0) {
            return false;
        }
        i += 1;
    }
    PINNED.len() == LABELS.len() && PINNED.len() < u8::MAX as usize
}

const _: () = assert!(agree(), "Qwen pins and src/qwen/labels.rs name different tiers");
