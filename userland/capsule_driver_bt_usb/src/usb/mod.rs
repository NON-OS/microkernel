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

mod bind;
mod choose;
mod limit;
mod ops;
mod pipes;

// The client call and the wire constants of driver.xhci0 are nonos_usbnet's
// own files, compiled here: that crate keeps them private, and a copy would
// drift from the one the network drivers use.
#[path = "../../../nonos_usbnet/src/xhci/call.rs"]
mod call;
#[allow(dead_code)]
#[path = "../../../nonos_usbnet/src/xhci/wire.rs"]
mod wire;

pub use bind::{bind, Bound};
pub use pipes::XhciPipes;
