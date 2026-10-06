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

pub fn inquiry() -> ([u8; 16], u8) {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x12;
    cdb[4] = 36;
    (cdb, 6)
}

/// START STOP UNIT with START set: spins up a medium that reports NOT READY
/// until it is told to start, as USB-SATA bridges and some sticks do.
pub fn start_unit() -> ([u8; 16], u8) {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x1B;
    cdb[4] = 0x01;
    (cdb, 6)
}

pub fn read_capacity10() -> ([u8; 16], u8) {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x25;
    (cdb, 10)
}

/// READ CAPACITY(16) (SERVICE ACTION IN(16), service action 0x10), asking
/// for the 32 bytes SBC defines: the 64-bit last LBA and the block length.
pub fn read_capacity16() -> ([u8; 16], u8) {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x9E;
    cdb[1] = 0x10;
    cdb[10..14].copy_from_slice(&(super::CAPACITY16_DATA_LEN as u32).to_be_bytes());
    (cdb, 16)
}

pub fn read10(lba: u32, blocks: u16) -> ([u8; 16], u8) {
    block_cdb(0x28, lba, blocks)
}

pub fn write10(lba: u32, blocks: u16) -> ([u8; 16], u8) {
    block_cdb(0x2A, lba, blocks)
}

/// READ(16) and WRITE(16): a 64-bit LBA and a 32-bit block count, for a
/// device past the 2^32 blocks the 10-byte forms address.
pub fn read16(lba: u64, blocks: u32) -> ([u8; 16], u8) {
    block_cdb16(0x88, lba, blocks)
}

pub fn write16(lba: u64, blocks: u32) -> ([u8; 16], u8) {
    block_cdb16(0x8A, lba, blocks)
}

fn block_cdb16(op: u8, lba: u64, blocks: u32) -> ([u8; 16], u8) {
    let mut cdb = [0u8; 16];
    cdb[0] = op;
    cdb[2..10].copy_from_slice(&lba.to_be_bytes());
    cdb[10..14].copy_from_slice(&blocks.to_be_bytes());
    (cdb, 16)
}

fn block_cdb(op: u8, lba: u32, blocks: u16) -> ([u8; 16], u8) {
    let mut cdb = [0u8; 16];
    cdb[0] = op;
    cdb[2..6].copy_from_slice(&lba.to_be_bytes());
    cdb[7..9].copy_from_slice(&blocks.to_be_bytes());
    (cdb, 10)
}
