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

//! Locating a TPM whose TIS (FIFO) registers are memory mapped: QEMU's virt
//! board puts `tpm-tis-device` on its platform bus, where only the tree says
//! where it is (`compatible = "tcg,tpm-tis-mmio"`; recalled, unverified).

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

const TIS_MMIO: &[&[u8]] = &[b"tcg,tpm-tis-mmio"];

/// The register window, base and length, of the first enabled memory-mapped
/// TIS TPM in the tree, or `None` for a board with none.
pub fn find(fdt: &Fdt) -> Result<Option<(u64, u64)>, FdtError> {
    for found in fdt.devices(TIS_MMIO) {
        let (node, _) = found?;
        if let Some(window) = fdt.mmio(&node, 0)? {
            return Ok(Some(window));
        }
    }
    Ok(None)
}
