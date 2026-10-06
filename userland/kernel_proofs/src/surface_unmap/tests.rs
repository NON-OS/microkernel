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

use super::unmap_rule::{on_unmap, OnUnmap};

#[test]
fn a_surface_nobody_else_maps_is_freed_when_its_owner_unmaps_all_of_it() {
    assert_eq!(on_unmap(false, true), OnUnmap::Free);
}

#[test]
fn a_part_unmapped_surface_nobody_else_maps_only_stops_being_attachable() {
    assert_eq!(on_unmap(false, false), OnUnmap::Detach);
}

#[test]
fn a_surface_another_process_maps_waits_for_its_last_holder() {
    assert_eq!(on_unmap(true, true), OnUnmap::Orphan);
    assert_eq!(on_unmap(true, false), OnUnmap::Keep);
}

#[test]
fn only_a_whole_unmap_nobody_else_sees_frees_at_once() {
    for attached in [false, true] {
        for whole in [false, true] {
            let free = on_unmap(attached, whole) == OnUnmap::Free;
            assert_eq!(free, !attached && whole, "attached {attached} whole {whole}");
        }
    }
}
