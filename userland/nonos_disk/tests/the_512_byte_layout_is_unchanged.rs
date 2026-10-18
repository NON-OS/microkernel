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

//! The bytes an install writes on a disk of 512-byte sectors, pinned on the
//! tree before the writer lays a disk out on larger blocks: every sector a
//! disk holds after an install at three sizes, with an empty store and a
//! carried one, and every write of `every_write_is_whole_sectors.rs`'s run,
//! each as a count and a digest (`common/golden.rs`). A moved disk digest
//! is a change in the bytes on an installed disk; a moved write digest is a
//! change in the order or the size of the writes, which a disk digest need
//! not see. Either needs the sign-off before it lands. Once it has
//! that, regenerate by running
//! `cargo test --test the_512_byte_layout_is_unchanged`: a failure prints
//! what the tree writes now, spelled as the tables below are.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/golden.rs"]
mod golden;
#[path = "common/golden_inputs.rs"]
mod golden_inputs;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/strict_sink.rs"]
mod strict_sink;

use golden::{spelled, Golden};
use nonos_disk::{install, StoreImage, MIN_DISK_SECTORS};

const SIZES: [u64; 3] = [MIN_DISK_SECTORS, MIN_DISK_SECTORS + 12_345, (1 << 32) + (1 << 21) + 5];
/// Each size with an empty store, then each with the carried one.
const DISKS: [Golden; 6] = [
    (24126, 0x0b9423ce354dab190e67ea591737e99c),
    (24126, 0x2e130480d804f78a00fe500f3a32aef5),
    (24126, 0x0b2045bb81c6a758d6a0f65fa6469e82),
    (24882, 0xd28326d3cf9a1d2f7d719c51793f2d8f),
    (24882, 0xcd04caccd212eb307f3d313352cebf1e),
    (24882, 0xafb80830d7dda7c3f8f3596bdbc5ad39),
];
const WRITES: Golden = (53, 0xf298edf90dc83e721b20265118890d6d);

#[test]
fn every_sector_an_install_writes_is_as_pinned() {
    let f = files::files();
    let mut got = Vec::new();
    for store in [StoreImage::empty(), golden_inputs::carried()] {
        for total in SIZES {
            let mut disk = mem_disk::MemDisk::new(total);
            install(&mut disk, &files::image(&f), store.clone(), entropy::ENTROPY, &mut |_| {})
                .unwrap();
            got.push(golden::sectors(&disk));
        }
    }
    assert!(got == DISKS, "the tree writes\n{}", spelled(&got));
}

#[test]
fn every_write_goes_out_as_pinned() {
    let got = golden::writes(&golden_inputs::strict_writes());
    assert!(got == WRITES, "the tree writes\n{}", spelled(&[got]));
}
