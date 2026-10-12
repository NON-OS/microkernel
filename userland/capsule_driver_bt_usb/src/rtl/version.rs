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

//! Read Local Version Information (Vol 4, Part E, 7.4.1), asked before
//! anything is reset or set, so the ROM's own answer is what is matched.

use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::cursor::Reader;
use nonos_bt_wire::hci::opcode as op;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version {
    pub hci_version: u8,
    pub hci_revision: u16,
    pub company: u16,
    pub lmp_subversion: u16,
}

pub fn read<T: Transport>(host: &mut Host<T>) -> Result<Version, HciError> {
    let ret = host.command(op::READ_LOCAL_VERSION, &[])?;
    let mut r = Reader::new(&ret);
    let (hci_version, hci_revision, _lmp_version) = (r.u8()?, r.u16()?, r.u8()?);
    let (company, lmp_subversion) = (r.u16()?, r.u16()?);
    Ok(Version { hci_version, hci_revision, company, lmp_subversion })
}
