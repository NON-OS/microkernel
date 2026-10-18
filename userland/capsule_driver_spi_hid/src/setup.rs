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

//! Setup: spi0 opened for this driver, and the firmware asked which device
//! on spi0's bus is HID over SPI and how it is run. With neither, there is
//! nothing here to drive.

use crate::link::{set, Link};

pub fn run() -> Result<Link, &'static str> {
    let (port, key) = crate::spi::open()?;
    let choice = crate::acpi::find(key)?;
    let link = Link { port, choice };
    set(&link);
    Ok(link)
}
