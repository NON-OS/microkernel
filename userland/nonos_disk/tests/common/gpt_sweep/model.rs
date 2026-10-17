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

//! What the reader owes a disk, from UEFI 2.10 5.2.3, 5.3.2 and 5.3.3 (as
//! recalled, unverified) and the reader's own limits: 512-byte sectors, an
//! array of at most 128 KiB between the header and FirstUsableLBA.

use super::crc::crc32;
use super::disk::Disk;

/// A partition as the table names it: type, unique GUID, first and last.
pub type Part = ([u8; 16], [u8; 16], u64, u64);

/// A protective MBR: 55h AAh, and an EEh record starting at LBA 1.
pub fn protects(mbr: &[u8]) -> bool {
    let at_one = |e: &[u8]| e[4] == 0xEE && e[8..12] == [1, 0, 0, 0];
    mbr[510..512] == [0x55, 0xAA] && mbr[446..510].chunks(16).any(at_one)
}

/// The header the reader believes: its fields hold together, and the
/// CRC over HeaderSize bytes with the CRC field zero matches.
pub fn header(d: &Disk) -> Option<(u64, u64, u64, u64, u64)> {
    let size = d.get(12, 4) as usize;
    if &d.header[..8] != b"EFI PART" || !(92..=512).contains(&size) || d.get(24, 8) != 1 {
        return None;
    }
    let (first, last, lba) = (d.get(40, 8), d.get(48, 8), d.get(72, 8));
    let (n, esize) = (d.get(80, 4), d.get(84, 4));
    let array_end = lba.checked_add((n * esize).div_ceil(512));
    let shaped = esize >= 128 && esize.is_power_of_two() && n >= 1 && n * esize <= 128 * 1024;
    let placed = lba >= 2 && array_end.is_some_and(|end| end <= first);
    let ranged = first <= last && last < d.sectors;
    let sealed =
        crc32(&[&d.header[..16], &[0; 4], &d.header[20..size]].concat()) == d.get(16, 4) as u32;
    (shaped && placed && ranged && sealed).then_some((first, last, lba, n, esize))
}

/// The partitions a believed header's array names: the array CRC over
/// every entry; unused entries (type all zero) and those not wholly inside
/// the usable range, or ending before they start, left out; and nothing at
/// all when two that are kept share a sector.
pub fn table(d: &Disk) -> Option<Vec<Part>> {
    let (first, last, _, n, esize) = header(d)?;
    let array = &d.array[..(n * esize) as usize];
    if crc32(array) != d.get(88, 4) as u32 {
        return None;
    }
    let field = |e: &[u8], at: usize| u64::from_le_bytes(e[at..at + 8].try_into().unwrap());
    let kept: Vec<Part> = array
        .chunks(esize as usize)
        .map(|e| {
            (e[..16].try_into().unwrap(), e[16..32].try_into().unwrap(), field(e, 32), field(e, 40))
        })
        .filter(|p: &Part| p.0 != [0; 16] && first <= p.2 && p.2 <= p.3 && p.3 <= last)
        .collect();
    for (i, p) in kept.iter().enumerate() {
        if kept[i + 1..].iter().any(|q| p.2 <= q.3 && q.2 <= p.3) {
            return None;
        }
    }
    Some(kept)
}
