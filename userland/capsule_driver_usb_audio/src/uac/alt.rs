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

//! A playback alternate setting, and the descriptor numbers that find it.

pub(super) const INTERFACE: u8 = 0x04;
pub(super) const ENDPOINT: u8 = 0x05;
pub(super) const CS_INTERFACE: u8 = 0x24;
pub(super) const CS_ENDPOINT: u8 = 0x25;
pub(super) const CLASS_AUDIO: u8 = 0x01;
pub(super) const SUBCLASS_STREAMING: u8 = 0x02;
/// bInterfaceProtocol of a USB Audio 1.0 function; 2.0 says 0x20 (as recalled, unverified).
pub(super) const UAC1: u8 = 0x00;
pub(super) const ISOCH: u8 = 0x01;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Playback {
    pub interface: u8,
    pub alt: u8,
    pub ep_address: u8,
    /// bmAttributes bits 3:2: 1 asynchronous, 2 adaptive, 3 synchronous.
    pub sync: u8,
    pub max_packet: u16,
    pub b_interval: u8,
    /// The endpoint takes SET_CUR of its sampling frequency.
    pub rate_control: bool,
}

#[derive(Default)]
pub(super) struct Alt {
    pub p: Playback,
    pub streaming: bool,
    pub pcm: bool,
    pub format: bool,
}

impl Alt {
    pub fn usable(&self) -> bool {
        self.streaming && self.pcm && self.format && self.p.ep_address != 0
    }
}
