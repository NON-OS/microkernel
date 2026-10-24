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
//! A streaming alternate setting, either way, and the descriptor numbers
//! that find it.

pub(super) const INTERFACE: u8 = 0x04;
pub(super) const ENDPOINT: u8 = 0x05;
pub(super) const CS_INTERFACE: u8 = 0x24;
pub(super) const CS_ENDPOINT: u8 = 0x25;
pub(super) const CLASS_AUDIO: u8 = 0x01;
pub(super) const SUBCLASS_STREAMING: u8 = 0x02;
/// bInterfaceProtocol of a USB Audio 1.0 and 2.0 function (as recalled,
/// unverified).
pub(super) const UAC1: u8 = 0x00;
pub(super) const UAC2: u8 = 0x20;
pub(super) const ISOCH: u8 = 0x01;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Stream {
    pub interface: u8,
    pub alt: u8,
    pub ep_address: u8,
    /// bmAttributes bits 3:2: 1 asynchronous, 2 adaptive, 3 synchronous.
    pub sync: u8,
    pub max_packet: u16,
    pub b_interval: u8,
    /// A 1.0 endpoint that takes SET_CUR of its sampling frequency.
    pub rate_control: bool,
    /// Channels in a frame: 2 played; 1 or 2 captured.
    pub channels: u8,
    /// 1 or 2. A 2.0 stream's rate is its terminal's clock's (clock.rs).
    pub version: u8,
    /// bTerminalLink: the terminal the alternate streams through.
    pub terminal: u8,
}
