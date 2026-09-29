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

use spin::Once;

use super::backend::Backend;

static SELECTED: Once<Backend> = Once::new();

pub fn selected() -> Backend {
    *SELECTED.call_once(|| {
        if matches!(crate::hardware::nvme_capsule::capacity(), Ok(s) if s > 0) && nvme_sectors_fit()
        {
            return Backend::Nvme;
        }
        if matches!(crate::hardware::ahci_capsule::capacity(), Ok(s) if s > 0) {
            return Backend::Ahci;
        }
        Backend::VirtioBlk
    })
}

/// Every caller addresses 512-byte sectors. A namespace formatted with
/// larger blocks would read eight times the bytes asked for at eight times
/// the offset, so it is passed over by name rather than misaddressed.
fn nvme_sectors_fit() -> bool {
    match crate::hardware::nvme_capsule::identify_namespace() {
        Ok(ns) if ns.lba_size != 512 => {
            crate::log::warn!(
                "[BLOCK] NVMe namespace {} uses {}-byte blocks; only 512 is addressed, not used",
                ns.nsid,
                ns.lba_size
            );
            false
        }
        _ => true,
    }
}
