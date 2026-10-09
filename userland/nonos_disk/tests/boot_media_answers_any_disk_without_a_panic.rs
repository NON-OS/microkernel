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

//! Whether a disk is the boot media is asked of every disk in the machine,
//! whatever a previous owner or a hostile stick left on it: a partition
//! table, a boot sector, a FAT, directories. From a fixed xorshift seed, a
//! real boot stick with random bytes of its table, boot sector, FAT and
//! directories damaged, and wholly random first sectors, each asked with
//! the loader's bytes and with a record naming a partition. The answer is
//! a yes or a no, never a panic, and some damage still leaves the stick
//! found, so the walk reached the file.

#[path = "common/fat16_sectors.rs"]
mod fat16_sectors;
#[path = "common/fat16_stick.rs"]
mod fat16_stick;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use fat16_stick::{stick, SIGNATURE, SIZE, START};
use mem_disk::MemDisk;
use nonos_disk::{is_boot_media, BlockSink, BootEvidence, BootPartition, SIGNATURE_MBR, TABLE_MBR};
use nonos_disk::{SIGNATURE_GUID, TABLE_GPT};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
}

fn record(rng: &mut Rng) -> BootPartition {
    let mut signature = [0u8; 16];
    signature[..4].copy_from_slice(&SIGNATURE);
    let gpt = rng.below(2) == 0;
    BootPartition {
        number: [1, 2, 4, 5, 0, u32::MAX][rng.below(6) as usize],
        table: if gpt { TABLE_GPT } else { TABLE_MBR },
        signature_type: if gpt { SIGNATURE_GUID } else { SIGNATURE_MBR },
        start_lba: [START, 0, u64::MAX, rng.next()][rng.below(4) as usize],
        size_lba: [SIZE, 0, u64::MAX][rng.below(3) as usize],
        signature,
    }
}

#[test]
fn any_damage_to_a_boot_stick_is_answered_without_a_panic() {
    let loader = b"MZ\x90\x00 the running loader's first bytes";
    let mut rng = Rng(0x5DEE_CE66_D1CE_4E5D);
    let mut found = 0;
    for partitioned in [true, false] {
        let real = stick(partitioned, loader);
        let lbas: Vec<u64> = real.written.keys().copied().collect();
        for _ in 0..50_000 {
            let mut disk = MemDisk { sectors: real.sectors, written: real.written.clone() };
            for _ in 0..1 + rng.below(6) {
                let lba = lbas[rng.below(lbas.len() as u64) as usize];
                let sector = disk.written.get_mut(&lba).expect("written");
                let at = rng.below(512) as usize;
                sector[at] = if rng.below(2) == 0 { rng.next() as u8 } else { sector[at] ^ 0x80 };
            }
            let by_loader = BootEvidence {
                partition: None,
                loader_size: loader.len() as u64,
                loader_head: loader,
            };
            found += is_boot_media(&mut disk, &by_loader) as usize;
            let by_record = BootEvidence { partition: Some(record(&mut rng)), ..by_loader };
            let _ = is_boot_media(&mut disk, &by_record);
        }
    }
    assert!(found > 10_000, "the damaged sticks found {found} times");
}

#[test]
fn any_first_sectors_at_all_are_answered_without_a_panic() {
    let loader = b"MZ loader";
    let mut rng = Rng(0x0123_4567_89AB_CDEF);
    for _ in 0..100_000 {
        let sectors = [64, 4096, 1 << 32, rng.below(1 << 40) + 2][rng.below(4) as usize];
        let mut disk = MemDisk::new(sectors);
        for lba in 0..rng.below(4) + 1 {
            let mut s = [0u8; 512];
            s.iter_mut().for_each(|b| *b = rng.next() as u8);
            if lba == 1 && rng.below(2) == 0 {
                s[..8].copy_from_slice(b"EFI PART");
            }
            if rng.below(2) == 0 {
                s[510..].copy_from_slice(&[0x55, 0xAA]);
                s[0] = 0xEB;
                s[11..13].copy_from_slice(&512u16.to_le_bytes());
            }
            disk.write_at(lba, &s).expect("whole sectors");
        }
        let ev = BootEvidence { partition: None, loader_size: 9, loader_head: loader };
        let _ = is_boot_media(&mut disk, &ev);
        let ev = BootEvidence { partition: Some(record(&mut rng)), ..ev };
        let _ = is_boot_media(&mut disk, &ev);
    }
}
