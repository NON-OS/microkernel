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

//! The controller driver as the driver's files reach it, played on the
//! host: a device's report descriptor (`report_desc`), the reports its endpoints have ready
//! (`interrupt`), and the slot and address calls counted (`ops`). Only what
//! the included enumeration, binding and poll files call.

/// The speeds the Interval below is told apart by.
#[path = "../../../capsule_driver_xhci/src/contexts/ep0.rs"]
pub mod ep0;
pub mod interrupt;
/// The controller driver's own endpoint Interval, which the poll loop's
/// pace must follow.
#[path = "../../../capsule_driver_xhci/src/contexts/interval.rs"]
pub mod interval;
mod ops;
pub mod report_desc;
/// A control request's setup stage, as the driver's files build it.
#[path = "../../../capsule_driver_usb_hid/src/xhci/ops/setup.rs"]
pub mod setup;

pub use interrupt::interrupt_in;
pub use ops::{control_transfer, disable_slot, enable_slot, XhciClientError};
pub use setup::Setup;
