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

/// State the DMA posture on an architecture whose IOMMU bring-up does not.
///
/// x86_64 states it from its own bring-up, after the vendor is chosen from the
/// ACPI tables. aarch64 has no SMMU backend yet, so nothing confines a device:
/// the line says so in the shape x86 uses, and the broker reprints it each
/// time a grant goes out unconfined, so a capsule with DMA never runs under a
/// posture the log did not state.
pub(crate) fn report_dma_posture() {
    #[cfg(target_arch = "aarch64")]
    {
        use crate::memory::iommu::{report_posture, select_vendor};
        let _ = select_vendor();
        crate::sys::serial::println(
            b"[IOMMU] no SMMU backend in this kernel; IOMMU domains refused; DMA is unrestricted",
        );
        report_posture();
    }
}
