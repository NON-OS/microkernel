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

use super::version::GicVersion;

/// Every distributor we know, GICv3 first. GIC-400 is the Pi 4's; the
/// Cortex-A15 string is what QEMU `virt,gic-version=2` writes.
pub(in crate::arch::fdt::find) const NEEDLES: &[&[u8]] = &[
    b"arm,gic-v3",
    b"arm,gic-400",
    b"arm,cortex-a15-gic",
    b"arm,cortex-a9-gic",
    b"arm,cortex-a7-gic",
];

pub(super) fn version(needle: usize) -> GicVersion {
    match needle {
        0 => GicVersion::V3,
        _ => GicVersion::V2,
    }
}
