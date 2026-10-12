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

//! What the namespace says about ports: each USB port's connectability,
//! connector and physical location (_UPC, _PLD), so internal ports (a
//! built-in camera, a Bluetooth module) are told from the ones a user can
//! reach; and which PCIe root ports face the outside of the machine.

mod external;
mod panel;
mod pld;
mod port;
mod upc;
mod usb;

pub use external::{facing, Facing, DMA_PROPERTY_UUID, EXTERNAL_FACING_UUID};
pub use panel::Panel;
pub use pld::{pld, Pld};
pub use port::{Reach, UsbPort};
pub use upc::{upc, Upc, PROPRIETARY};
pub use usb::usb_ports;
