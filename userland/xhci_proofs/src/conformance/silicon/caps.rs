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

//! Capability lists laid out in a window, for the capability tests.

use nonos_devmodel::FakeBar;

/// A capability list: `(offset, dw0, dw2)`, each pointing at the next.
pub(super) fn lay_out_caps(bar: &FakeBar, caps: &[(usize, u32, u32)]) {
    for (i, &(off, id_and_rev, dw2)) in caps.iter().enumerate() {
        let next = caps.get(i + 1).map_or(0, |n| ((n.0 - off) / 4) as u32);
        bar.present32(off, id_and_rev | (next << 8));
        bar.present32(off + 8, dw2);
    }
}

pub(super) fn supported_protocol(major: u32) -> u32 {
    2 | (major << 24)
}
