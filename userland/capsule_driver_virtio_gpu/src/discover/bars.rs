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

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO};
use nonos_virtio::{BarInfo, Bars};

/// The broker's BAR list in the shared transport's terms.
pub(super) fn bars(r: &DeviceRecord) -> Bars {
    let mut out = [BarInfo::ABSENT; 6];
    for (slot, bar) in out.iter_mut().zip(r.bars.iter()) {
        *slot = match bar.kind {
            BAR_KIND_MMIO => BarInfo::mmio(bar.size),
            BAR_KIND_PIO => BarInfo::io(bar.size),
            _ => BarInfo::ABSENT,
        };
    }
    out
}
