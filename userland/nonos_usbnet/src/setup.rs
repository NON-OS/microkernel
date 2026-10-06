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

//! A control request's setup packet (USB 2.0, 9.3) and the standard and
//! class requests every USB network driver sends.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Setup {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
}

/// bmRequestType: device-to-host, and the type and recipient bits.
pub const DIR_IN: u8 = 0x80;
pub const TYPE_CLASS: u8 = 0x20;
pub const TYPE_VENDOR: u8 = 0x40;
pub const TO_INTERFACE: u8 = 0x01;

const GET_DESCRIPTOR: u8 = 0x06;
const SET_CONFIGURATION: u8 = 0x09;
const SET_INTERFACE: u8 = 0x0B;
const LANG_EN_US: u16 = 0x0409;

impl Setup {
    pub const fn new(request_type: u8, request: u8, value: u16, index: u16) -> Self {
        Self { request_type, request, value, index }
    }

    /// GET_DESCRIPTOR of `kind` at `index` (USB 2.0, 9.4.3).
    pub const fn get_descriptor(kind: u8, index: u8) -> Self {
        let lang = if kind == crate::desc::STRING { LANG_EN_US } else { 0 };
        Self::new(DIR_IN, GET_DESCRIPTOR, (kind as u16) << 8 | index as u16, lang)
    }

    pub const fn set_configuration(value: u8) -> Self {
        Self::new(0, SET_CONFIGURATION, value as u16, 0)
    }

    /// SET_INTERFACE: select `alt` of `interface` (USB 2.0, 9.4.10).
    pub const fn set_interface(interface: u8, alt: u8) -> Self {
        Self::new(TO_INTERFACE, SET_INTERFACE, alt as u16, interface as u16)
    }

    /// A class request to `interface`, in or out.
    pub const fn class(dir_in: bool, request: u8, value: u16, interface: u8) -> Self {
        let dir = if dir_in { DIR_IN } else { 0 };
        Self::new(dir | TYPE_CLASS | TO_INTERFACE, request, value, interface as u16)
    }
}
