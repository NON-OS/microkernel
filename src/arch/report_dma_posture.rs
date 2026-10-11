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

/// Bring up the IOMMU and state the DMA posture on an architecture whose
/// IOMMU bring-up does not happen elsewhere.
///
/// x86_64 does both from its own bring-up, after the vendor is chosen from
/// the ACPI tables. aarch64 brings up the SMMUv3 the device tree names; with
/// none, or one the driver leaves off, nothing confines a device, and the
/// line says why in the shape x86 uses. Either way the broker reprints the
/// posture each time a grant goes out unconfined.
pub(crate) fn report_dma_posture() {
    #[cfg(target_arch = "aarch64")]
    {
        use crate::memory::iommu::{report_posture, select_vendor};
        use crate::sys::serial::Line;
        match crate::arch::aarch64::iommu::smmu::bring_up() {
            Ok(()) => crate::sys::serial::println(
                b"[SMMU] arm-smmu-v3 in service; every stream aborts until a domain claims it",
            ),
            Err(reason) => Line::new()
                .str(b"[IOMMU] ")
                .str(reason.as_bytes())
                .str(b"; IOMMU domains refused; DMA is unrestricted")
                .end(),
        }
        let _ = select_vendor();
        report_posture();
    }
}
