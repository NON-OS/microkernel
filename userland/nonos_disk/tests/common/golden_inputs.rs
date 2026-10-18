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

//! What the golden digests are taken over, besides the image in
//! `files.rs`: a store holding what an install carries, and the write list
//! of the run `every_write_is_whole_sectors.rs` makes.

use nonos_disk::{install, NonosImage, StoreBuilder, StoreImage};

use super::entropy::ENTROPY;
use super::strict_sink::StrictSink;

/// Setup's answers and marker, a wallpaper vfs streams, a Linux guest and
/// a capsule with a proof each, an empty file and a file the guest reads,
/// added the way `gather` adds them but from fixed files, so a change to
/// what gather picks moves no digest.
pub fn carried() -> StoreImage {
    let bytes =
        |len: usize, seed: u8| (0..len).map(|i| (i as u8).wrapping_mul(31) ^ seed).collect();
    let files: [(&str, Vec<u8>); 9] = [
        ("/nonos/setup/answers", b"NSA1\x02\xfb\0".to_vec()),
        ("/nonos/setup/done", b"NSD1".to_vec()),
        ("/Wallpapers/a.jpg", bytes(9_000, 1)),
        ("/linux/bin/guest", bytes(300_007, 2)),
        ("/linux/bin/guest.zk_trailer.bin", bytes(5_000, 3)),
        ("/capsules/a.elf", bytes(70_001, 4)),
        ("/capsules/a.manifest.bin", bytes(512, 5)),
        ("/capsules/empty.bin", Vec::new()),
        ("/linux/usr/lib/data.bin", bytes(1_000, 6)),
    ];
    let mut store = StoreBuilder::new();
    files.iter().for_each(|(name, data)| store.add(name, data).unwrap());
    store.finish()
}

/// Every write, in order, of an install onto an 8 GiB disk with a loader
/// that fills its clusters exactly and a kernel that does not.
pub fn strict_writes() -> Vec<(u64, usize)> {
    let (boot_efi, kernel_bin) = (vec![0xAAu8; 14_225_408], vec![0x55u8; 89_971_099]);
    let image = NonosImage {
        boot_efi: &boot_efi,
        kernel_bin: &kernel_bin,
        boot_cfg: b"x=1\n",
        boot_trailer: &[0x11; 70_011],
        boot_root: &[0x22; 186],
        kernel_approval: None,
    };
    let mut sink =
        StrictSink { sectors: (8u64 << 30) / 512, writes: Vec::new(), flushed_after: None };
    let _ = install(&mut sink, &image, StoreImage::empty(), ENTROPY, &mut |_| {});
    sink.writes
}
