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

//! Which driver a disk is, for a record that names its kind and not what
//! it holds: the install source's in-use disk (D13b).

use super::backend::Backend;
use super::rows::{AHCI, NVME, USB_MSC, VIRTIO_BLK};

/// A driver with a wire of its own (rows/), or the block wire's endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    VirtioBlk,
    Ahci,
    Nvme,
    UsbMsc,
    Wire,
}

impl Backend {
    pub fn kind(&self) -> Kind {
        let is = |row| core::ptr::eq(self.disk, row);
        if is(&VIRTIO_BLK) {
            Kind::VirtioBlk
        } else if is(&AHCI) {
            Kind::Ahci
        } else if is(&NVME) {
            Kind::Nvme
        } else if is(&USB_MSC) {
            Kind::UsbMsc
        } else {
            Kind::Wire
        }
    }
}
