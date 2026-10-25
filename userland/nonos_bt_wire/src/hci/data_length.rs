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

//! LE Data Length Extension (Vol 4, Part E, 7.8.33 and 7.8.46; Vol 6,
//! Part B, 4.5.10): a controller with the feature can carry up to 251
//! octets of payload in one link-layer PDU instead of 27. The host reads
//! the most the controller supports, then asks for it on each connection.
//! Both directions of every value are held to the specification's range.

use super::acl::HANDLE_MAX;
use super::opcode::Opcode;
use crate::cursor::Reader;
use crate::WireError;
use core::ops::RangeInclusive;

/// Vol 4, Part E, 7.8.33.
pub const LE_SET_DATA_LENGTH: Opcode = Opcode::new(0x08, 0x0022);
/// Vol 4, Part E, 7.8.46.
pub const LE_READ_MAXIMUM_DATA_LENGTH: Opcode = Opcode::new(0x08, 0x002f);

/// The payload octets one PDU may carry, 0x001B to 0x00FB.
pub const OCTETS: RangeInclusive<u16> = 27..=251;
/// The microseconds one PDU may take, 0x0148 to 0x4290.
pub const TIME_US: RangeInclusive<u16> = 328..=17040;

/// The controller's maximum, from LE Read Maximum Data Length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaxDataLength {
    pub tx_octets: u16,
    pub tx_time: u16,
    pub rx_octets: u16,
    pub rx_time: u16,
}

impl MaxDataLength {
    /// The return parameters after the status octet; a value outside the
    /// specification's range is refused.
    pub fn parse(ret: &[u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(ret);
        let m =
            Self { tx_octets: r.u16()?, tx_time: r.u16()?, rx_octets: r.u16()?, rx_time: r.u16()? };
        let ok = OCTETS.contains(&m.tx_octets) && OCTETS.contains(&m.rx_octets);
        (ok && TIME_US.contains(&m.tx_time) && TIME_US.contains(&m.rx_time))
            .then_some(m)
            .ok_or(WireError::Reserved)
    }
}

/// LE Set Data Length's parameters: the connection, and the octets and
/// time to ask for, each within the specification's range.
pub fn set_data_length(handle: u16, octets: u16, time_us: u16) -> Result<[u8; 6], WireError> {
    if handle > HANDLE_MAX || !OCTETS.contains(&octets) || !TIME_US.contains(&time_us) {
        return Err(WireError::Reserved);
    }
    let mut p = [0u8; 6];
    p[..2].copy_from_slice(&handle.to_le_bytes());
    p[2..4].copy_from_slice(&octets.to_le_bytes());
    p[4..].copy_from_slice(&time_us.to_le_bytes());
    Ok(p)
}
