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

//! The owner's development opt-in to unconfined DMA (plan/50 P2), the cargo
//! feature nonos-dev-unconfined-dma: refused at compile time beside
//! nonos-production or nonos-release, and named on the IOMMU posture line.

#[cfg(all(
    feature = "nonos-dev-unconfined-dma",
    any(feature = "nonos-production", feature = "nonos-release")
))]
compile_error!(
    "nonos-dev-unconfined-dma lets a device no IOMMU domain confines reach all of \
     memory; it is for development images and never part of a production build."
);

pub(crate) const OPT_IN: bool = cfg!(feature = "nonos-dev-unconfined-dma");
pub(crate) const PRODUCTION: bool =
    cfg!(any(feature = "nonos-production", feature = "nonos-release"));

/// The opt-in holds: built in, and never beside a production build.
pub(crate) const fn holds() -> bool {
    OPT_IN && !PRODUCTION
}

/// The posture line's tail: empty unless the opt-in is built in.
pub(crate) const fn posture_tail() -> &'static [u8] {
    if OPT_IN {
        b", dev-unconfined-dma=1"
    } else {
        b""
    }
}
