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

//! What the IOMMU guarantees at the moment of asking.

use super::vendor::IommuVendor;

/*
 * Every field is a guarantee in force now, never a capacity the hardware
 * reports and the kernel does not use. While `enforcing` is false no device
 * is translated, so every other field is zero or false. `enforcing` says the
 * kernel's tables are live; it does not say a device is held to its grants,
 * since bring-up's identity domain maps all of RAM for every enumerated
 * device.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IommuCapabilities {
    pub vendor: IommuVendor,
    /// Every remapping unit the firmware described translates with this kernel's tables.
    pub enforcing: bool,
    /// IOVAs below 2^n can be mapped into a domain and are translated.
    pub address_width_bits: u8,
    /// The kernel programmed interrupt remapping.
    pub interrupt_remapping: bool,
    /// Bit k set: `IommuDomain::map` installs leaves of 2^k bytes.
    pub page_sizes: u64,
    /// Leaves ask for snooping and the unit honours the request.
    pub snoop_control: bool,
    /*
     * Size of the domain id space the unit and the kernel's domain table both
     * accept. A capacity, not what is left: ids are never reused, id 0 is
     * never handed out and bring-up's identity domain holds id 1.
     */
    pub domain_count: u32,
}

impl IommuCapabilities {
    /// No guarantee at all, reported under `vendor`.
    pub const fn none_in_force(vendor: IommuVendor) -> Self {
        Self {
            vendor,
            enforcing: false,
            address_width_bits: 0,
            interrupt_remapping: false,
            page_sizes: 0,
            snoop_control: false,
            domain_count: 0,
        }
    }
}
