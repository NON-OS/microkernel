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

use super::describe;
use crate::arch::fdt::find::reserved_dma::Refusal;
use crate::arch::fdt::Fdt;
use crate::sys::serial::Line;

/// The tree's reserved DMA ranges, kept for the broker's claims. Nothing
/// here refuses the tree: what cannot be read is said, and the devices it
/// would have covered are attached without reserved ranges, as before.
pub(super) fn populate(fdt: &Fdt) -> Result<(), &'static [u8]> {
    let reason: &[u8] = match crate::arch::aarch64::iommu::reserved::describe(fdt) {
        Ok(_) => return Ok(()),
        Err(Refusal::Tree(e)) => describe(e),
        Err(Refusal::TooMany) => b"more entries or regions than kept",
    };
    Line::new().str(b"[IOMMU] reserved DMA ranges not read: ").str(reason).end();
    Ok(())
}
