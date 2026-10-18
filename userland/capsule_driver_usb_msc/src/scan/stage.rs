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

//! What the search last did (`report`). The numbers are the wire's.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Stage {
    /// driver.xhci0 has not registered yet.
    NoXhci = 1,
    /// The controller answers and no port has a device connected.
    NoPort = 2,
    /// Another class driver is addressing the port.
    Busy = 3,
    /// No device slot, or the device would not take an address.
    Address = 4,
    /// Its configuration descriptor did not read. (A serial number that
    /// does not read is none, and the device is bound all the same.)
    Config = 5,
    /// No SCSI-transparent Bulk-Only interface; `aux` is the first
    /// interface's class, subclass and protocol.
    NotStorage = 6,
    /// SET_CONFIGURATION or the bulk pipes were refused.
    Configure = 7,
    /// Not ready: INQUIRY or TEST UNIT READY did not succeed in its time,
    /// or the transport failed three commands in a row (`pace`); or the
    /// device served was let go when the check after a failed request found
    /// it so (`release`), or after three failed requests in a row (`keep`).
    NotReady = 8,
    /// READ CAPACITY(10) failed.
    Capacity = 9,
    /// Bound: `blocks` of `aux` bytes.
    Bound = 10,
    /// The device served left its port; it is served again only if it
    /// comes back (scan/identity.rs).
    Gone = 11,
    /// No unit had a medium: an empty card reader.
    NoMedium = 12,
    /// Not the device served before it was let go, so not taken.
    Other = 13,
    /// Nothing is served again this boot: the medium served changed, its
    /// size did, or its card was taken out (`detail` E_NOMEDIUM), or the
    /// device was let go with no serial number to tell it by.
    Changed = 14,
}
