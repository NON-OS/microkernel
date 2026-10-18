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

//! The Union functional descriptor (CDC 1.2, 5.2.3.8, as recalled,
//! unverified): subtype 06h, bMasterInterface at 3, the first
//! bSlaveInterface at 4. The master must be the communications interface
//! it follows and the slave a CDC Data interface of the same configuration;
//! a Union naming anything else makes no function.

use nonos_usbnet::desc::{find_functional, Interface, CLASS_CDC_DATA};

const UNION: u8 = 0x06;

pub fn union_slave(raw: &[u8], comm: u8, ifs: &[Interface]) -> Option<u8> {
    let u = find_functional(raw, comm, UNION, 5)?;
    let (master, slave) = (u[3], u[4]);
    let data = |i: &Interface| i.number == slave && i.class == CLASS_CDC_DATA;
    (master == comm && ifs.iter().any(data)).then_some(slave)
}
