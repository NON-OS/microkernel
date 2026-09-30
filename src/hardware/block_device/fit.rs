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

//! Whether a backend's disk can be addressed in 512-byte sectors.

use super::backend::Backend;
use super::nvme_fit::nvme_sectors_fit;

/// Every caller addresses 512-byte sectors. A disk with larger logical
/// blocks is passed over by name rather than misaddressed.
pub(super) fn sectors_fit(backend: Backend) -> bool {
    match backend {
        Backend::Nvme => nvme_sectors_fit(),
        Backend::UsbMsc => usb_msc_sectors_fit(),
        Backend::Ahci | Backend::VirtioBlk => true,
    }
}

fn usb_msc_sectors_fit() -> bool {
    match crate::hardware::usb_msc_capsule::geometry() {
        Ok((_, block_len)) if block_len != 512 => {
            crate::log::warn!(
                "[BLOCK] USB mass-storage device uses {}-byte blocks; only 512 is addressed, not used",
                block_len
            );
            false
        }
        _ => true,
    }
}
