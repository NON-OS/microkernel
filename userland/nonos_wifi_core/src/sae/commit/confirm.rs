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

//! CN(KCK, send-confirm, scalar1, element1, scalar2, element2).

use super::*;

/// CN(KCK, send-confirm, scalar1, element1, scalar2, element2).
pub fn confirm(
    kck: &[u8; 32],
    send_confirm: u16,
    scalar1: &[u8; LEN],
    element1: &[u8; 2 * LEN],
    scalar2: &[u8; LEN],
    element2: &[u8; 2 * LEN],
) -> [u8; 32] {
    hmac_sha256_parts(kck, &[&send_confirm.to_le_bytes(), scalar1, element1, scalar2, element2])
}
