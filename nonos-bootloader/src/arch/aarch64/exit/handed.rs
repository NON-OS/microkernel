// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Which regions of the final EFI map the loader hands the kernel: its own
//! allocations, UEFI memory types LoaderCode and LoaderData. Pure, so it is
//! held on the host (loader_arm_proofs).

const LOADER_CODE: u32 = 1;
const LOADER_DATA: u32 = 2;

/// Whether a region of this type is one the loader hands the kernel.
pub fn handed_over(memory_type: u32) -> bool {
    memory_type == LOADER_CODE || memory_type == LOADER_DATA
}
