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

//! The plan around the stack and the vault.

use super::helpers::{covered, exact, run, span};

#[test]
fn a_heap_that_overlaps_nothing_is_wiped_whole() {
    let heap = span(0x1000, 0x9000);
    assert_eq!(run(heap, &[span(0x10_0000, 0x11_0000), span(0x0, 0x800)]), vec![heap]);
}

#[test]
fn the_running_stack_is_never_wiped_wherever_it_sits() {
    let heap = span(0x1000, 0x5000);
    for (a, b) in
        [(0x0, 0x1800), (0x2000, 0x3000), (0x4800, 0x6000), (0x0, 0x9000), (0x1000, 0x5000)]
    {
        exact(heap, &[span(a, b)]);
    }
}

#[test]
fn the_vault_inside_the_heap_is_wiped_around_every_kept_span() {
    // The key vault's map lives in the heap; an address standing for it, at
    // either end, in the middle, and between kept spans, is always covered.
    let heap = span(0x1000, 0x9000);
    let keep = [span(0x2000, 0x2400), span(0x5000, 0x6000), span(0x8f00, 0x9100)];
    let spans = run(heap, &keep);
    for vault in [0x1000, 0x1fff, 0x2400, 0x4000, 0x6000, 0x8eff] {
        assert!(covered(&spans, vault), "vault at {vault:#x}: {spans:?}");
    }
    exact(heap, &keep);
}
