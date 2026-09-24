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
//! Setting alpha without touching the colour.

use crate::theme::derive::{opaque, with_alpha};

#[test]
fn alpha_replaces_only_the_alpha() {
    assert_eq!(with_alpha(0xFF35_C4E2, 0x80), 0x8035_C4E2);
    assert_eq!(with_alpha(0x0035_C4E2, 0xFF), 0xFF35_C4E2);
}
#[test]
fn a_ground_is_forced_opaque() {
    assert_eq!(opaque(0x0012_3456), 0xFF12_3456);
    assert_eq!(opaque(0xFF12_3456), 0xFF12_3456);
}
