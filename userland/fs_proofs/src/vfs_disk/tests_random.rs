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

//! The store's bytes as an attacker or a failing disk would choose them,
//! from a fixed xorshift seed so a failure replays. Whatever the bytes, the
//! decode and the load return; what they keep is in bounds; and nothing is
//! served that its own descriptor does not vouch for.

use nonos_disk_map::{digest16, valid_name, MAX_ENTRIES, MAX_TOTAL_BYTES, STORE_BASE_LBA};
use nonos_libc::disk;

use super::fixture::{bytes, image, refs, Layout, BASE, ENTRY_LEN, HEADER_LEN, NAME_LEN};
use super::run::load_counting;
use crate::vfs_blk::store_patch::store_toc::{decode, Window};

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

    fn fill(&mut self, out: &mut [u8]) {
        out.iter_mut().for_each(|b| *b = self.next() as u8);
    }
}

const END: u64 = 64 << 20;

/// Both sides of the budget, for the decode on its own.
const BOUNDARY_LENS: [u64; 7] = [0, 1, 512, 4096, MAX_TOTAL_BYTES, MAX_TOTAL_BYTES + 1, u64::MAX];

/// For a store that is loaded: the decode takes a length of the whole budget
/// and the load would then read and digest sixty MiB of zeros each time,
/// which proves nothing the decode proof above does not.
const LOADED_LENS: [u64; 6] = [0, 1, 512, 4096, MAX_TOTAL_BYTES + 1, u64::MAX];

/// One descriptor, each field either noise or a value near a boundary the
/// decode checks, so both sides of every check are reached often. `lens`
/// are the lengths it picks from when it does not pick noise.
fn descriptor(rng: &mut Rng, floor: u64, lens: &[u64], out: &mut [u8]) {
    rng.fill(out);
    const NAMES: [&[u8]; 8] =
        [b"/a", b"/capsules/x.elf", b"/a/../b", b"rel", b"/a//b", b"", b"/a\nb", b"/nonos/k"];
    if rng.below(4) != 0 {
        out[..NAME_LEN].fill(0);
        let name = NAMES[rng.below(NAMES.len() as u64) as usize];
        out[..name.len()].copy_from_slice(name);
    }
    let near = [0, floor, floor - 512, END, END - 512, u64::MAX, BASE, floor + 1];
    if rng.below(4) != 0 {
        // Past u64::MAX on purpose: a hostile descriptor's offset wraps.
        let at = near[rng.below(near.len() as u64) as usize].wrapping_add(512 * rng.below(4));
        out[NAME_LEN..NAME_LEN + 8].copy_from_slice(&at.to_le_bytes());
    }
    if rng.below(4) != 0 {
        let len = lens[rng.below(lens.len() as u64) as usize];
        out[NAME_LEN + 8..NAME_LEN + 16].copy_from_slice(&len.to_le_bytes());
    }
}

#[test]
fn any_table_decodes_without_panic_and_keeps_only_what_is_in_bounds() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let window = Window { base: BASE, end: END };
    for _ in 0..200_000 {
        let count = rng.below(MAX_ENTRIES as u64 + 6) as usize;
        let table = HEADER_LEN + ENTRY_LEN * count;
        let floor = BASE + (table.div_ceil(512) * 512) as u64;
        let len = if rng.below(8) == 0 { rng.below(table as u64 + 64) as usize } else { table };
        let mut toc = vec![0u8; len];
        rng.fill(&mut toc);
        for slot in 0..count {
            let at = HEADER_LEN + ENTRY_LEN * slot;
            if at + ENTRY_LEN <= len {
                descriptor(&mut rng, floor, &BOUNDARY_LENS, &mut toc[at..at + ENTRY_LEN]);
            }
        }
        let Ok(got) = decode(&toc, count, window) else {
            assert!(len < table, "a whole table of {count} refused");
            continue;
        };
        assert_eq!(got.entries.len() + got.refused, count);
        let mut total = 0u64;
        for (i, e) in got.entries.iter().enumerate() {
            assert!(i == 0 || got.entries[i - 1].slot < e.slot);
            assert!(e.slot < count && valid_name(&e.name), "{}", e.name);
            assert!(e.offset % 512 == 0 && e.offset >= floor && e.offset + e.len <= END);
            total += e.len;
        }
        assert!(total <= MAX_TOTAL_BYTES);
    }
}

/// A small store with every byte of its header, table and payloads chosen
/// at random, mostly with a header the decode will read on.
fn random_store(rng: &mut Rng) -> (Vec<u8>, usize) {
    let count = rng.below(7) as usize;
    let mut img = vec![0u8; 512 * (2 + rng.below(12) as usize)];
    rng.fill(&mut img);
    if rng.below(8) != 0 {
        img[..8].copy_from_slice(b"NONOSTR1");
        img[8..12].copy_from_slice(&1u32.to_le_bytes());
        img[12..16].copy_from_slice(&(count as u32).to_le_bytes());
    }
    let floor = BASE + 1024;
    for slot in 0..count {
        let at = HEADER_LEN + ENTRY_LEN * slot;
        descriptor(rng, floor, &LOADED_LENS, &mut img[at..at + ENTRY_LEN]);
        if rng.below(2) == 0 {
            let near = BASE + 512 * (2 + rng.below(10));
            img[at + NAME_LEN..at + NAME_LEN + 8].copy_from_slice(&near.to_le_bytes());
            img[at + NAME_LEN + 8..at + NAME_LEN + 16]
                .copy_from_slice(&rng.below(900).to_le_bytes());
        }
        if rng.below(2) == 0 {
            img[at + NAME_LEN + 16..at + ENTRY_LEN].fill(0);
        }
    }
    (img, count)
}

/// The descriptor in `img` that vouches for serving `data` as `name`: one
/// with that name whose digest is the data's, or none.
fn vouched(img: &[u8], name: &str, data: &[u8]) -> bool {
    img[HEADER_LEN..].chunks(ENTRY_LEN).filter(|e| e.len() == ENTRY_LEN).any(|e| {
        let end = e[..NAME_LEN].iter().position(|&b| b == 0).unwrap_or(NAME_LEN);
        let digest = &e[NAME_LEN + 16..ENTRY_LEN];
        &e[..end] == name.as_bytes() && (digest == digest16(data) || digest == [0u8; 16])
    })
}

#[test]
fn any_store_loads_without_panic_and_serves_only_what_a_descriptor_vouches_for() {
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    let mut served = 0usize;
    for _ in 0..100_000 {
        let (img, count) = random_store(&mut rng);
        disk::reset();
        disk::put(STORE_BASE_LBA, &img);
        let Ok((got, refused)) = load_counting() else { continue };
        assert_eq!(got.len() + refused, count);
        for (name, data) in &got {
            assert!(valid_name(name), "{name:?}");
            assert!(vouched(&img, name, data), "{name} served bytes no descriptor vouches for");
        }
        served += got.len();
    }
    assert!(served > 10_000, "the generator reached the loader's happy path {served} times");
}

#[test]
fn random_bit_flips_in_a_real_store_never_serve_a_file_changed() {
    let files: Vec<(String, Vec<u8>)> = vec![
        (String::from("/nonos/setup/answers"), bytes(1, 300)),
        (String::from("/capsules/demo.elf"), bytes(2, 2000)),
        (String::from("/nonos/empty"), Vec::new()),
    ];
    let img = image(&refs(&files), Layout::Packer);
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for _ in 0..100_000 {
        let mut out = img.clone();
        for _ in 0..1 + rng.below(4) {
            let byte = if rng.below(2) == 0 { rng.below(512) } else { rng.below(img.len() as u64) };
            out[byte as usize] ^= 1 << rng.below(8);
        }
        disk::reset();
        disk::put(STORE_BASE_LBA, &out);
        let Ok((got, refused)) = load_counting() else { continue };
        assert!(got.len() + refused <= 128);
        for (name, data) in &got {
            assert!(files.iter().any(|(_, d)| d == data), "{name} served bytes no file had");
        }
    }
}
