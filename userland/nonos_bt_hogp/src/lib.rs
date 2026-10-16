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

//! HID over GATT (the HID over GATT Profile, HOGP 1.0, and the HID Service,
//! HIDS 1.0) on the Attribute Protocol and GATT of the Bluetooth Core
//! Specification 5.4, Vol 3, Parts F and G. This host reads the boot
//! keyboard and boot mouse reports, whose layout the USB HID specification
//! fixes (HID 1.11, Appendix B), and takes a keyboard to report mode only
//! when its Report Map parses within fixed bounds and lays out its keys.
//! Anything less keeps boot mode, which needs no map, so a crafted map can
//! cost a peripheral its extra keys but never its typing. Once the reports
//! are on, it reads the device's Battery Levels too (BAS 1.0), without
//! ever holding up or ending HID for them.
//!
//! The ATT bearer must already be encrypted: the HID service's
//! characteristics need it (HOGP 1.0, 4.1), and a peripheral that answers
//! with an authentication or encryption error ends discovery so the caller
//! can pair first.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod error;
pub mod host;
pub mod report;
pub mod server;
pub mod uuids;

pub use error::HogpError;
