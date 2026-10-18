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

//! A PCIe port whose _DSD says ExternalFacingPort: the kernel reads the
//! buses behind it from the port and flags every function there, which the
//! IOMMU then keeps out of the identity domain. DmaProperty rides along.

use super::publish::function;
use crate::boot::{evaluator, path, Acpi};
use crate::sys::log::Line;
use crate::sys::ports::external_port;
use nonos_aml::namespace::Id;
use nonos_aml::ports::facing;

pub(super) fn declare(acpi: &mut Acpi, dev: Id) {
    let f = facing(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev);
    if !f.external {
        return;
    }
    let line = Line::new(b"external port ").str(path(&acpi.aml, dev).as_bytes());
    let Some(at) = function(acpi, dev) else { return line.str(b": no function").end() };
    let dma = f.dma_property.is_some_and(|v| v != 0);
    match external_port(at, dma) {
        Ok(n) => line.str(b": ").dec(n).str(b" functions flagged").end(),
        Err(e) => line.str(b": refused ").dec(e.unsigned_abs()).end(),
    }
}
