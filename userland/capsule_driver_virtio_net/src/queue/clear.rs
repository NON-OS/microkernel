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

use nonos_virtio::Mmio;

/// Zero `len` bytes of `region` from `from`. The writes are volatile and
/// bounds checked: bytes past the region are not reached.
pub fn zero(region: Mmio, from: usize, len: usize) {
    for at in from..from.saturating_add(len) {
        region.w8(at, 0);
    }
}
