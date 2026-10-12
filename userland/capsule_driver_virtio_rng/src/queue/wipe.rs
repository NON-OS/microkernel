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

//! Emptying a DMA view with writes the compiler keeps: the volatile
//! accessors of the bounds-checked region.

use nonos_virtio::Mmio;

pub fn wipe(m: Mmio) {
    let words = m.len() / 4 * 4;
    for at in (0..words).step_by(4) {
        m.w32(at, 0);
    }
    for at in words..m.len() {
        m.w8(at, 0);
    }
}
