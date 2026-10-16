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

//! What bring-up learns of the controller, from the return parameters the
//! status octet has already been taken off. Octets past the fields read are
//! left alone, so a controller of a later version that returns more is
//! still understood.

use crate::error::HciError;
use nonos_bt_wire::addr::BdAddr;
use nonos_bt_wire::cursor::Reader;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Controller {
    pub hci_version: u8,
    pub hci_revision: u16,
    pub lmp_version: u8,
    pub company: u16,
    pub lmp_subversion: u16,
    pub addr: BdAddr,
    /// The most data octets one ACL packet to the controller may carry.
    pub acl_len: u16,
    /// How many ACL packets the controller holds at once.
    pub acl_total: u16,
    /// The LE features (Vol 6, Part B, 4.6), bit 0 first.
    pub le_features: u64,
}

/// Read Local Version Information (Vol 4, Part E, 7.4.1): HCI version and
/// revision, LMP version, company identifier, LMP subversion.
pub fn version(ret: &[u8]) -> Result<(u8, u16, u8, u16, u16), HciError> {
    let mut r = Reader::new(ret);
    Ok((r.u8()?, r.u16()?, r.u8()?, r.u16()?, r.u16()?))
}

/// Read BD_ADDR (Vol 4, Part E, 7.4.6).
pub fn bd_addr(ret: &[u8]) -> Result<BdAddr, HciError> {
    Ok(BdAddr(Reader::new(ret).array()?))
}

/// LE Read Buffer Size (Vol 4, Part E, 7.8.2): the LE ACL packet length and
/// count. Both zero means the controller shares its BR/EDR buffers, which
/// Read Buffer Size then gives.
pub fn le_buffers(ret: &[u8]) -> Result<Option<(u16, u16)>, HciError> {
    let mut r = Reader::new(ret);
    let (len, total) = (r.u16()?, u16::from(r.u8()?));
    Ok((len != 0 || total != 0).then_some((len, total)))
}

/// Read Buffer Size (Vol 4, Part E, 7.4.5): ACL length, synchronous
/// length, ACL count, synchronous count; the ACL pair is kept.
pub fn buffers(ret: &[u8]) -> Result<(u16, u16), HciError> {
    let mut r = Reader::new(ret);
    let (len, _sync_len, total) = (r.u16()?, r.u8()?, r.u16()?);
    Ok((len, total))
}
