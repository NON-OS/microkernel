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

//! Every bound the write parser holds, at the bound and one past it: the
//! last sector, the sector count, the frame a byte long or short, an LBA
//! whose sum would wrap.

use super::frames::{req, rw_body};
use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::protocol::{E_INVAL, E_MSGSIZE, E_NXIO, RW_HEADER_LEN};
use crate::server::write::parse_write;

#[test]
fn write_parse_enforces_the_exact_boundaries() {
    let cap = 1_000u64;
    let n1 = SECTOR_SIZE;
    let good = rw_body(cap - 1, 1, n1);
    let hdr = req((RW_HEADER_LEN + n1) as u32);
    assert_eq!(parse_write(cap, &hdr, &good), Ok((cap - 1, 1, n1)));
    assert_eq!(parse_write(cap, &req(RW_HEADER_LEN as u32 + 1), &good).err(), Some(E_MSGSIZE));
    let mut long = good.clone();
    long.push(0);
    assert_eq!(parse_write(cap, &hdr, &long).err(), Some(E_MSGSIZE));
    let mut short = good.clone();
    short.pop();
    assert_eq!(parse_write(cap, &hdr, &short).err(), Some(E_MSGSIZE));
    assert_eq!(parse_write(cap, &hdr, &rw_body(cap, 1, n1)).err(), Some(E_NXIO));
    assert_eq!(
        parse_write(cap, &req(RW_HEADER_LEN as u32), &rw_body(0, 0, 0)).err(),
        Some(E_INVAL)
    );
    let m = MAX_SECTORS_PER_REQUEST;
    let over = rw_body(0, m + 1, (m + 1) as usize * SECTOR_SIZE);
    assert_eq!(parse_write(cap, &req(over.len() as u32), &over).err(), Some(E_INVAL));
    let wrap = rw_body(u64::MAX, 1, n1);
    assert_eq!(parse_write(cap, &req(wrap.len() as u32), &wrap).err(), Some(E_INVAL));
}
