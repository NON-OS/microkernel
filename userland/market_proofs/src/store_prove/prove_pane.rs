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

//! The detail pane asks its proof once: on a guest the question and its
//! two keys were painted twice, one under the other, because the pane was
//! painted twice in a row.

const DETAIL: &str = include_str!("../../../capsule_app_store/src/store/ui/detail.rs");

#[test]
fn the_detail_pane_paints_the_proof_once() {
    assert_eq!(DETAIL.matches("proof_pane::paint(").count(), 1);
}
