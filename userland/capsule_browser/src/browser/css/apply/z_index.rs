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

use crate::browser::css::computed::Computed;

/* Computed.z holds z-index offset by Z_SET, so 0 stays free for auto: an
 * integer z-index, 0 included, makes a stacking context and auto does not
 * (CSS 2.1 9.9.1). */
const Z_SET: i32 = 1000;

pub(super) fn apply_z_index(c: &mut Computed, value: &str) {
    let v = value.trim();
    if v.eq_ignore_ascii_case("auto") {
        c.z = 0;
    } else if let Ok(z) = v.parse::<i32>() {
        c.z = Z_SET + z.clamp(-999, 999);
    }
}

impl Computed {
    /// z-index: None for auto, else the integer, clamped to -999..=999.
    pub(crate) fn z_index(&self) -> Option<i32> {
        (self.z != 0).then(|| self.z - Z_SET)
    }
}
