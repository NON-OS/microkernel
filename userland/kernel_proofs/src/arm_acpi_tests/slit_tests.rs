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

//! SLIT's matrix taken only when it holds what it says: 10 on the
//! diagonal, nothing reserved off it, and as many bytes as its count.

use super::srat_build::slit;
use crate::arm_acpi::numa::slit_matrix::{matrix, MAX_LOCALITIES};

#[test]
fn slit_matrices() {
    let two = slit(2, &[10, 21, 21, 10]);
    assert_eq!(matrix(&two), Some((2, vec![10, 21, 21, 10])));
    assert_eq!(matrix(&slit(2, &[10, 255, 255, 10])).map(|m| m.0), Some(2), "unreachable");
    assert_eq!(matrix(&slit(2, &[10, 21, 21, 11])), None, "the diagonal is 10");
    assert_eq!(matrix(&slit(2, &[10, 9, 21, 10])), None, "below 10 is reserved");
    assert_eq!(matrix(&slit(2, &[10, 21, 21])), None, "short of its matrix");
    assert_eq!(matrix(&slit(0, &[])), None);
    assert_eq!(matrix(&slit(MAX_LOCALITIES as u64 + 1, &[])), None);
    assert_eq!(matrix(&two[..40]), None);
}
