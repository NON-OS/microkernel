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

//! Events that carry a list. Number of Completed Packets (Vol 4, Part E,
//! 7.7.19) holds a count then that many (handle, packets) pairs; LE
//! Advertising Report (Vol 4, Part E, 7.7.65.2) holds a count then that many
//! reports, each its event type, address type, address, data length, data
//! and RSSI. A list whose count and octets disagree is refused whole.

use alloc::vec::Vec;

use crate::addr::{AddrType, BdAddr};
use crate::cursor::Reader;
use crate::error::WireError;

/// The (handle, completed packets) pairs.
pub fn completed_packets(params: &[u8]) -> Result<Vec<(u16, u16)>, WireError> {
    let mut r = Reader::new(params);
    let n = r.u8()?;
    let mut out = Vec::with_capacity(usize::from(n));
    for _ in 0..n {
        out.push((r.u16()? & 0x0fff, r.u16()?));
    }
    r.finish()?;
    Ok(out)
}

pub struct AdvReport<'a> {
    pub event_type: u8,
    pub addr_type: AddrType,
    pub addr: BdAddr,
    pub data: &'a [u8],
    pub rssi: i8,
}

/// The reports of one LE Advertising Report subevent.
pub fn adv_reports(sub: &[u8]) -> Result<Vec<AdvReport<'_>>, WireError> {
    let mut r = Reader::new(sub);
    let n = r.u8()?;
    let mut out = Vec::with_capacity(usize::from(n));
    for _ in 0..n {
        let (event_type, addr_type) = (r.u8()?, AddrType::from_report(r.u8()?)?);
        let addr = BdAddr(r.array()?);
        let len = usize::from(r.u8()?);
        // Advertising data is at most 31 octets (Vol 3, Part C, 11).
        if len > 31 {
            return Err(WireError::TooLong);
        }
        let data = r.bytes(len)?;
        out.push(AdvReport { event_type, addr_type, addr, data, rssi: r.u8()? as i8 });
    }
    r.finish()?;
    Ok(out)
}
