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

//! The six ops on one disk.

use super::reply::{self, BODY_AT};
use super::rw;
use crate::constants::ata::SECTOR_BYTES;
use crate::constants::buffer::DATA_BYTES;
use crate::disk::Disk;
use crate::io::{self, prd, Command, Taskfile, E_IO};
use crate::setup::Driver;
use crate::wire::{
    encode_geometry, encode_identify, Request, E_NODEV, E_NXIO, GEOMETRY_LEN, IDENTIFY_LEN,
    MEDIA_PRESENT, MEDIUM_ATA, RW_HEADER_LEN,
};

pub fn geometry(disk: &Disk, req: &Request, tx: &mut [u8]) {
    let flags = MEDIA_PRESENT;
    encode_geometry(
        &mut tx[BODY_AT..],
        disk.id.sectors,
        SECTOR_BYTES as u32,
        DATA_BYTES as u32,
        flags,
    );
    reply::body(tx, req, GEOMETRY_LEN);
}

/// The model as the disk gives it, and the serial without the leading
/// spaces some disks pad it with.
pub fn identify(disk: &Disk, req: &Request, tx: &mut [u8]) {
    let serial = &disk.id.serial;
    let start = serial.iter().position(|&c| c != b' ').unwrap_or(serial.len());
    match encode_identify(&disk.id.model, &serial[start..], MEDIUM_ATA) {
        Some(b) => {
            tx[BODY_AT..BODY_AT + IDENTIFY_LEN].copy_from_slice(&b);
            reply::body(tx, req, IDENTIFY_LEN);
        }
        None => reply::status(tx, req, E_NODEV),
    }
}

pub fn read(d: &mut Driver, disk: &Disk, req: &Request, body: &[u8], tx: &mut [u8]) {
    let r = match rw::parse(body, false, disk.id.sectors) {
        Ok(r) => r,
        Err(e) => return reply::status(tx, req, e),
    };
    match run(d, disk, Command::Read, r) {
        Ok(()) => {
            let n = r.bytes();
            tx[BODY_AT..BODY_AT + n].copy_from_slice(d.data(n));
            reply::body(tx, req, n);
        }
        Err(e) => reply::status(tx, req, e),
    }
}

pub fn write(d: &mut Driver, disk: &Disk, req: &Request, body: &[u8], tx: &mut [u8]) {
    let r = match rw::parse(body, true, disk.id.sectors) {
        Ok(r) => r,
        Err(e) => return reply::status(tx, req, e),
    };
    let n = r.bytes();
    d.data(n).copy_from_slice(&body[RW_HEADER_LEN..RW_HEADER_LEN + n]);
    let status = run(d, disk, Command::Write, r).err().unwrap_or(0);
    reply::status(tx, req, status);
}

pub fn flush(d: &mut Driver, disk: &Disk, req: &Request, tx: &mut [u8]) {
    let status = match (Taskfile::flush(disk.drive, disk.id.flush), d.channel(disk)) {
        (None, _) => 0,
        (Some(tf), Some(ch)) => io::flush(&ch, disk.drive, &tf).err().unwrap_or(0),
        (Some(_), None) => E_IO,
    };
    reply::status(tx, req, status);
}

/// One transfer through the buffer: the task file, the descriptor table
/// for exactly the bytes moved, the engine run.
fn run(d: &mut Driver, disk: &Disk, cmd: Command, r: rw::Range) -> Result<(), i32> {
    let tf = Taskfile::dma(cmd, disk.drive, r.lba, r.count, disk.id.lba48).ok_or(E_NXIO)?;
    let ch = d.channel(disk).ok_or(E_IO)?;
    let data = d.data_dev();
    prd::build(data, r.bytes(), d.prd_table()).ok_or(E_IO)?;
    io::transfer(&ch, disk.drive, &tf, cmd, d.prd_dev())
}
