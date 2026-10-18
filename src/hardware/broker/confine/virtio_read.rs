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

//! The claim-time read of VIRTIO_F_ACCESS_PLATFORM (K5), as a sequence over
//! the common configuration's registers. Pure, and proven on the host
//! (kernel_proofs::virtio_confine). The driver must not inherit a select it
//! did not write, so device_feature_select is put back as it was found.

/// device_feature_select and device_feature, as offsets in the window.
pub(crate) const FEATURE_SELECT: u64 = 0x00;
pub(crate) const FEATURE: u64 = 0x04;

/// Bit 33, read as feature word 1, bit 1. None when the device does not
/// answer (all ones) or the select does not take: then the device is
/// refused. The select is restored on every path that changed it.
pub(crate) fn access_platform(
    read: impl Fn(u64) -> u32,
    mut write: impl FnMut(u64, u32),
) -> Option<bool> {
    let was = read(FEATURE_SELECT);
    if was == u32::MAX {
        return None;
    }
    write(FEATURE_SELECT, 1);
    let took = read(FEATURE_SELECT) == 1;
    let features = read(FEATURE);
    write(FEATURE_SELECT, was);
    (took && features != u32::MAX).then_some(features & (1 << 1) != 0)
}
