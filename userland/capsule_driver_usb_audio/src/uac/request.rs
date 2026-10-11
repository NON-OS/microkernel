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

//! The class request that sets the endpoint's sampling frequency (USB Audio
//! 1.0 5.2.3.2.3.1, as recalled, unverified): SET_CUR of SAMPLING_FREQ_CONTROL to the
//! endpoint, the rate in three little-endian bytes.

use nonos_usbnet::setup::Setup;

/// bmRequestType: host to device, class, endpoint.
const TO_ENDPOINT_CLASS: u8 = 0x22;
const SET_CUR: u8 = 0x01;
const SAMPLING_FREQ_CONTROL: u16 = 0x01;

pub fn set_rate(ep_address: u8, hz: u32) -> (Setup, [u8; 3]) {
    let s = Setup::new(TO_ENDPOINT_CLASS, SET_CUR, SAMPLING_FREQ_CONTROL << 8, ep_address as u16);
    let b = hz.to_le_bytes();
    (s, [b[0], b[1], b[2]])
}
