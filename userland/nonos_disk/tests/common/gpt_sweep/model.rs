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

//! What the reader owes each header, from UEFI 2.10 5.2.3, 5.3.2 and 5.3.3
//! (as recalled, unverified) and the reader's own limits: 512-byte
//! sectors, an array of at most 128 KiB.

use super::crc::crc32;
use super::disk::Disk;

/// A partition as the table names it: type, unique GUID, first and last.
pub type Part = ([u8; 16], [u8; 16], u64, u64);

/// A protective MBR: 55h AAh, and an EEh record starting at LBA 1.
pub fn protects(mbr: &[u8]) -> bool {
    let at_one = |e: &[u8]| e[4] == 0xEE && e[8..12] == [1, 0, 0, 0];
    mbr[510..512] == [0x55, 0xAA] && mbr[446..510].chunks(16).any(at_one)
}

/// Header `w` (0 the primary, 1 the backup) as the reader believes it: its
/// fields hold together, it names its own LBA (1, or the last) and the
/// backup names the primary at 1, its array lies in its room (after LBA 1
/// and before FirstUsableLBA, or after LastUsableLBA and before the backup
/// header), and its CRC over HeaderSize bytes, that field zero, matches.
pub fn header(d: &Disk, w: usize) -> Option<(u64, u64, u64, u64, u64)> {
    let (h, size) = (&d.heads[w], d.get(w, 12, 4) as usize);
    let own = d.get(w, 24, 8) == [1, d.sectors - 1][w] && (w == 0 || d.get(1, 32, 8) == 1);
    if &h[..8] != b"EFI PART" || !(92..=512).contains(&size) || !own {
        return None;
    }
    let (first, last, lba) = (d.get(w, 40, 8), d.get(w, 48, 8), d.get(w, 72, 8));
    let (n, esize) = (d.get(w, 80, 4), d.get(w, 84, 4));
    let array_end = lba.checked_add((n * esize).div_ceil(512));
    let (lo, hi) = [(2, first), (last.saturating_add(1), d.sectors - 1)][w];
    let shaped = esize >= 128 && esize.is_power_of_two() && n >= 1 && n * esize <= 128 * 1024;
    let placed = lba >= lo && array_end.is_some_and(|end| end <= hi);
    let ranged = first <= last && last < d.sectors;
    let sealed = crc32(&[&h[..16], &[0; 4], &h[20..size]].concat()) == d.get(w, 16, 4) as u32;
    (shaped && placed && ranged && sealed).then_some((first, last, lba, n, esize))
}

/// The partitions header `w`'s array names: its CRC over every entry;
/// unused entries (type zero) and those not wholly inside the usable range,
/// or ending before they start, left out; nothing when two kept ones share
/// a sector.
pub fn table(d: &Disk, w: usize) -> Option<Vec<Part>> {
    let (first, last, lba, n, esize) = header(d, w)?;
    let array = d.read(lba, (n * esize) as usize);
    if crc32(&array) != d.get(w, 88, 4) as u32 {
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
    let shared = |(i, p): (usize, &Part)| kept[i + 1..].iter().any(|q| p.2 <= q.3 && q.2 <= p.3);
    (!kept.iter().enumerate().any(shared)).then_some(kept)
}
