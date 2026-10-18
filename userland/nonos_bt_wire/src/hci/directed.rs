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

//! LE Directed Advertising Report (Vol 4, Part E, 7.7.65.11): what a
//! controller scanning with an extended filter policy (0x02 or 0x03,
//! 7.8.10) reports of a directed advertisement whose TargetA is a
//! resolvable private address it cannot resolve itself. A count, then
//! that many reports, each an event type, the advertiser's address type
//! and address, the target's address type and address, and an RSSI. The
//! host resolves the target to learn whether it is meant.

use alloc::vec::Vec;

use crate::addr::{AddrType, BdAddr};
use crate::cursor::Reader;
use crate::error::WireError;

/// ADV_DIRECT_IND, the only event type the event carries.
pub const ADV_DIRECT_IND: u8 = 0x01;
/// A random device address, the only target type the event carries.
const RANDOM_TARGET: u8 = 0x01;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedReport {
    pub addr_type: AddrType,
    pub addr: BdAddr,
    /// TargetA: the address the advertiser directed itself to.
    pub direct_addr: BdAddr,
    pub rssi: i8,
}

/// The reports of one subevent; a list with a reserved event type or
/// target type, or whose count and octets disagree, is refused whole.
pub fn directed_reports(sub: &[u8]) -> Result<Vec<DirectedReport>, WireError> {
    let mut r = Reader::new(sub);
    let n = r.u8()?;
    let mut out = Vec::with_capacity(usize::from(n));
    for _ in 0..n {
        if r.u8()? != ADV_DIRECT_IND {
            return Err(WireError::Reserved);
        }
        let (addr_type, addr) = (AddrType::from_report(r.u8()?)?, BdAddr(r.array()?));
        if r.u8()? != RANDOM_TARGET {
            return Err(WireError::Reserved);
        }
        let direct_addr = BdAddr(r.array()?);
        out.push(DirectedReport { addr_type, addr, direct_addr, rssi: r.u8()? as i8 });
    }
    r.finish()?;
    Ok(out)
}
