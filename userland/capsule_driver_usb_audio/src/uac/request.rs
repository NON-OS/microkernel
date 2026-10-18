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

//! The class requests that set a stream's rate. 1.0 (5.2.3.2.3.1, as
//! recalled, unverified): SET_CUR of SAMPLING_FREQ_CONTROL to the endpoint,
//! the rate in three little-endian bytes. 2.0: to the stream's clock.

use nonos_usbnet::setup::{Setup, DIR_IN, TO_INTERFACE, TYPE_CLASS};

use super::clock::Clock;

/// bmRequestType: host to device, class, endpoint.
const TO_ENDPOINT_CLASS: u8 = 0x22;
const SET_CUR: u8 = 0x01;
const SAMPLING_FREQ_CONTROL: u16 = 0x01;

pub fn set_rate(ep_address: u8, hz: u32) -> (Setup, [u8; 3]) {
    let s = Setup::new(TO_ENDPOINT_CLASS, SET_CUR, SAMPLING_FREQ_CONTROL << 8, ep_address as u16);
    let b = hz.to_le_bytes();
    (s, [b[0], b[1], b[2]])
}

/// 2.0 requests go to an entity in the AudioControl interface; CUR is the
/// same number either way (2.0 5.2.1 and 5.2.5.1, as recalled, unverified).
const CUR: u8 = 0x01;
const SAM_FREQ_CONTROL: u16 = 0x01;
const CLOCK_SELECTOR_CONTROL: u16 = 0x01;
const OUT: u8 = TYPE_CLASS | TO_INTERFACE;

fn at(c: &Clock, entity: u8) -> u16 {
    (entity as u16) << 8 | c.interface as u16
}

/// SET CUR of a 2.0 clock's sampling frequency, four little-endian bytes.
pub fn set_clock(c: &Clock, hz: u32) -> (Setup, [u8; 4]) {
    (Setup::new(OUT, CUR, SAM_FREQ_CONTROL << 8, at(c, c.id)), hz.to_le_bytes())
}

/// GET CUR of it, read back to see the rate the clock runs at.
pub fn get_clock(c: &Clock) -> Setup {
    Setup::new(DIR_IN | OUT, CUR, SAM_FREQ_CONTROL << 8, at(c, c.id))
}

/// SET CUR of the selector before it to its first input (pins count from 1).
pub fn select_first(c: &Clock, selector: u8) -> (Setup, [u8; 1]) {
    (Setup::new(OUT, CUR, CLOCK_SELECTOR_CONTROL << 8, at(c, selector)), [1])
}
