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

//! A member the pack tool writes fits the install role's heap: the largest
//! it cuts (PART_RAW, 128 MiB) at the largest chunk a list may name, with
//! room to spare; so libreoffice-common, 343 MB in one package, installs in
//! members of at most 70 MB. A member past the bound is refused before any
//! of it is read.

use nonos_app_image::{CHUNK_MAX, MAX_RAW};

use crate::install::image::room::needs;

const HEAP_RS: &str = include_str!("../../../capsule_linux/src/linux/heap.rs");
const SPLIT_PY: &str = include_str!("../../../../tools/nonos_linux_pack/image_split.py");
const HEAP: u64 = 320 << 20;

#[test]
fn the_heap_and_the_cut_are_the_ones_the_tree_names() {
    assert!(HEAP_RS.contains("const INSTALL_HEAP: usize = 320 << 20;"));
    assert!(SPLIT_PY.contains("PART_RAW = 128 << 20"));
}

#[test]
fn the_largest_member_the_pack_tool_writes_fits_with_room() {
    let need = needs(128 << 20, CHUNK_MAX);
    assert!(need <= HEAP - (16 << 20), "{need} bytes, past the heap less 16 MiB");
}

#[test]
fn a_member_at_the_formats_bound_does_not_fit_and_is_refused_up_front() {
    assert!(needs(MAX_RAW, CHUNK_MAX) > HEAP);
}

#[test]
fn libreoffices_members_fit() {
    // The raw sizes of libreoffice-common's six members in the image built
    // on 2026-11-02.
    for raw in [63_279_104u64, 58_136_064, 68_027_904, 63_572_480, 70_431_232, 17_133_568] {
        assert!(needs(raw, 4 << 20) < HEAP / 2, "{raw}");
    }
}
