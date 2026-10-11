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

//! The domain an enumerated function starts in. Pure, and
//! proven on the host (kernel_proofs::iommu_start). Never an identity map.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Start {
    /// The shared empty domain: every DMA faults until a capsule claims it.
    Blocked,
    /// Its own domain, holding only the memory firmware reserved for it.
    Reserved,
}

/// `reserved`: firmware listed reserved memory for it. `had`: a domain of
/// its own could be made and mapped. Any failure leaves it blocked.
pub const fn start_for(reserved: bool, had: bool) -> Start {
    if reserved && had {
        Start::Reserved
    } else {
        Start::Blocked
    }
}
