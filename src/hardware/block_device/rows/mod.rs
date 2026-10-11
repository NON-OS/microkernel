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

//! The drivers with a wire of their own, one row each. The block wire's
//! row lives with its client, in `wire`.

mod ahci;
mod nvme;
mod nvme_fit;
mod usb_msc;
mod usb_msc_fit;
mod virtio;

pub(super) use ahci::AHCI;
pub(super) use nvme::NVME;
pub(super) use usb_msc::USB_MSC;
pub(super) use virtio::VIRTIO_BLK;
