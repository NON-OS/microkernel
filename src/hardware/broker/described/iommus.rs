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

//! Which IOMMU a described device's DMA goes through, as the broker keeps
//! it: the firmware description's `iommus`, independent of how any arch
//! reads it.

/// Stream ids kept for one device.
pub const MAX_DESCRIBED_STREAMS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescribedIommus {
    /// Nothing says a unit translates its DMA: it is left unconfined, said.
    None,
    /// The unit, as the description names it, and the device's streams.
    Streams { unit: u32, sids: [u32; MAX_DESCRIBED_STREAMS], count: u8 },
    /// A description the kernel cannot honour: the claim is refused.
    Unreadable(&'static [u8]),
}

impl DescribedIommus {
    pub fn sids(&self) -> &[u32] {
        match self {
            Self::Streams { sids, count, .. } => {
                &sids[..usize::from(*count).min(MAX_DESCRIBED_STREAMS)]
            }
            _ => &[],
        }
    }
}
