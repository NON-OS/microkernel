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


//! Whether the broker lists an I2C host driver.i2c_pci may drive (i2c_host).

use super::i2c_host::i2c_host;
use crate::hardware::broker::list;

pub fn i2c_host_present() -> bool {
    list().iter().any(|r| {
        i2c_host(r.bus_kind, r.vendor, r.device, r.class, (r.pci_class, r.pci_subclass))
    })
}
