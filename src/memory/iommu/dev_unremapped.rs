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

//! The owner's development opt-in to interrupts without remapping (K3), the
//! cargo feature nonos-dev-unremapped-interrupts: a PCI device may be
//! claimed, and its MSI bound in compatibility format, where no unit
//! remaps. Refused at compile time beside nonos-production or
//! nonos-release, and named on the IOMMU posture line.

#[cfg(all(
    feature = "nonos-dev-unremapped-interrupts",
    any(feature = "nonos-production", feature = "nonos-release")
))]
compile_error!(
    "nonos-dev-unremapped-interrupts lets a device write a compatibility format \
     message to any CPU with any vector; it is for development images and never \
     part of a production build."
);

pub(crate) const OPT_IN: bool = cfg!(feature = "nonos-dev-unremapped-interrupts");

pub(crate) const fn posture_tail() -> &'static [u8] {
    if OPT_IN {
        b", dev-unremapped-interrupts=1"
    } else {
        b""
    }
}
