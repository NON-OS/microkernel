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

//! The six ops on one unit, each carried by `nonos_scsi`.

use nonos_scsi::{max_blocks, Hba};

use super::reply::{self, BODY_AT};
use super::rw;
use crate::say::trim;
use crate::units::Units;
use crate::wire::{
    encode_geometry, encode_identify, Request, E_NODEV, GEOMETRY_LEN, IDENTIFY_LEN, MEDIA_PRESENT,
    MEDIUM_SCSI, READ_ONLY, REMOVABLE, RW_HEADER_LEN,
};

/// A removable unit is asked again whether a medium is in and how large it
/// is, so a disc changed since the scan is served as it is now.
pub fn geometry<H: Hba>(u: &mut Units<H>, req: &Request, tx: &mut [u8]) {
    let slot = req.unit as usize;
    let Some(mut d) = u.disks[slot] else { return };
    if d.removable {
        let _ = nonos_scsi::refresh(&mut u.hba, &mut d);
        u.disks[slot] = Some(d);
    }
    let mut flags = 0;
    if d.medium {
        flags |= MEDIA_PRESENT;
    }
    if d.removable {
        flags |= REMOVABLE;
    }
    if d.read_only {
        flags |= READ_ONLY;
    }
    let max = max_blocks(&d, u.hba.buffer().len()) * d.block_bytes;
    encode_geometry(&mut tx[BODY_AT..], d.blocks, d.block_bytes, max, flags);
    reply::body(u.reply, tx, req, GEOMETRY_LEN);
}

/// The vendor and product as the model, the unit serial number as the
/// serial.
pub fn identify<H: Hba>(u: &mut Units<H>, req: &Request, tx: &mut [u8]) {
    let Some(d) = u.disk(req.unit) else { return };
    let vendor = trim(&d.vendor);
    let product = trim(&d.product);
    let mut model = [0u8; 25];
    model[..vendor.len()].copy_from_slice(vendor);
    model[vendor.len()] = b' ';
    model[vendor.len() + 1..vendor.len() + 1 + product.len()].copy_from_slice(product);
    let model = &model[..vendor.len() + 1 + product.len()];
    let model = if vendor.is_empty() { &model[1..] } else { model };
    match encode_identify(model, &d.serial[..d.serial_len], MEDIUM_SCSI) {
        Some(b) => {
            tx[BODY_AT..BODY_AT + IDENTIFY_LEN].copy_from_slice(&b);
            reply::body(u.reply, tx, req, IDENTIFY_LEN);
        }
        None => reply::status(u.reply, tx, req, E_NODEV),
    }
}

fn range<H: Hba>(
    u: &mut Units<H>,
    req: &Request,
    body: &[u8],
    write: bool,
) -> Result<(nonos_scsi::Disk, rw::Range), i32> {
    let d = u.disk(req.unit).ok_or(crate::wire::E_NXIO)?;
    let max = max_blocks(&d, u.hba.buffer().len());
    let r = rw::parse(body, write, d.blocks, d.block_bytes, max)?;
    Ok((d, r))
}

pub fn read<H: Hba>(u: &mut Units<H>, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (d, r) = match range(u, req, body, false) {
        Ok(v) => v,
        Err(e) => return reply::status(u.reply, tx, req, e),
    };
    match nonos_scsi::read(&mut u.hba, &d, r.lba, r.count) {
        Ok(n) => {
            tx[BODY_AT..BODY_AT + n].copy_from_slice(&u.hba.buffer()[..n]);
            reply::body(u.reply, tx, req, n);
        }
        Err(e) => reply::status(u.reply, tx, req, e),
    }
}

pub fn write<H: Hba>(u: &mut Units<H>, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (d, r) = match range(u, req, body, true) {
        Ok(v) => v,
        Err(e) => return reply::status(u.reply, tx, req, e),
    };
    let n = body.len() - RW_HEADER_LEN;
    u.hba.buffer()[..n].copy_from_slice(&body[RW_HEADER_LEN..]);
    let status = nonos_scsi::write(&mut u.hba, &d, r.lba, r.count).err().unwrap_or(0);
    reply::status(u.reply, tx, req, status);
}

/// A CD drive or a write-protected disk has nothing to flush; `nonos_scsi`
/// answers for them without a command.
pub fn flush<H: Hba>(u: &mut Units<H>, req: &Request, tx: &mut [u8]) {
    let Some(d) = u.disk(req.unit) else { return };
    let status = nonos_scsi::flush(&mut u.hba, &d).err().unwrap_or(0);
    reply::status(u.reply, tx, req, status);
}
