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

/*
 * The sectors of the FAT16 stick: its MBR, the volume's boot sector (512
 * bytes a sector, 4 a cluster, 1 reserved, 2 FATs of 32 sectors, 512 root
 * entries, 32768 sectors), and one directory entry.
 */

use crate::fat16_stick::{SIGNATURE, SIZE, START};

pub fn mbr() -> Vec<u8> {
    let mut mbr = vec![0u8; 512];
    mbr[440..444].copy_from_slice(&SIGNATURE);
    mbr[446 + 4] = 0x06;
    mbr[446 + 8..446 + 12].copy_from_slice(&(START as u32).to_le_bytes());
    mbr[446 + 12..446 + 16].copy_from_slice(&(SIZE as u32).to_le_bytes());
    mbr[510..512].copy_from_slice(&[0x55, 0xAA]);
    mbr
}

pub fn bpb() -> Vec<u8> {
    let mut bpb = vec![0u8; 512];
    bpb[0] = 0xEB;
    bpb[11..13].copy_from_slice(&512u16.to_le_bytes());
    bpb[13..17].copy_from_slice(&[4, 1, 0, 2]);
    bpb[17..19].copy_from_slice(&512u16.to_le_bytes());
    bpb[19..21].copy_from_slice(&(SIZE as u16).to_le_bytes());
    bpb[22..24].copy_from_slice(&32u16.to_le_bytes());
    bpb[510..512].copy_from_slice(&[0x55, 0xAA]);
    bpb
}

pub fn entry(name: &[u8; 11], attr: u8, cluster: u16, size: u32) -> [u8; 32] {
    let mut e = [0u8; 32];
    e[..11].copy_from_slice(name);
    e[11] = attr;
    e[26..28].copy_from_slice(&cluster.to_le_bytes());
    e[28..32].copy_from_slice(&size.to_le_bytes());
    e
}
