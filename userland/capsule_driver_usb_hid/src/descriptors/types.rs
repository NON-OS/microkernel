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

pub const DT_CONFIGURATION: u8 = 0x02;
pub const DT_INTERFACE: u8 = 0x04;
pub const DT_ENDPOINT: u8 = 0x05;
/// The HID class descriptor between an interface and its endpoints, which
/// gives the length of the interface's report descriptor (HID 1.11, 6.2.1).
pub const DT_HID: u8 = 0x21;
pub const DT_REPORT: u8 = 0x22;
pub const CLASS_HID: u8 = 0x03;
pub const SUBCLASS_BOOT: u8 = 0x01;
pub const PROTOCOL_KEYBOARD: u8 = 0x01;
pub const PROTOCOL_MOUSE: u8 = 0x02;
pub const EP_TRANSFER_INTERRUPT: u8 = 0x03;

/// How an interface's reports are read: the boot keyboard and mouse
/// layouts, or the layout its own report descriptor gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidKind {
    Keyboard = 1,
    Mouse = 2,
    Report = 3,
    /// Laid out by its report descriptor (pad/), never by its config.
    Gamepad = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct Interface {
    pub number: u8,
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
    /// wDescriptorLength of its report descriptor, 0 until its HID class
    /// descriptor gives one.
    pub report_len: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Endpoint {
    pub address: u8,
    pub attributes: u8,
    pub max_packet_size: u16,
    pub interval: u8,
}
