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

//! The read-back passes on an intact disk, counts every file byte, and
//! names the first bad sector after one byte deep inside the kernel image
//! is flipped. A verifier that cannot fail is not a verifier.

mod common;

use nonos_disk::{install, verify, WriteError, SECTOR_SIZE};

#[test]
fn verify_reads_back_and_catches_corruption() {
    let f = common::files();
    let mut disk = common::MemDisk::new(320);
    let image = common::image(&f);
    let r = install(&mut disk, &image, common::ENTROPY, &mut |_| {}).unwrap();

    let checked = verify(&mut disk, &r, &mut |_| {}).unwrap();
    assert_eq!(checked, image.total_bytes());
    // Device bytes include each run's zero pad to the end of its cluster.
    assert!(r.bytes_written >= image.total_bytes());

    let kernel = r.files.iter().find(|fr| fr.data.len() == f.kernel_bin.len()).unwrap();
    let victim_sector = kernel.lba + 4000;
    let o = victim_sector as usize * SECTOR_SIZE + 17;
    disk.bytes[o] ^= 0x01;
    assert_eq!(
        verify(&mut disk, &r, &mut |_| {}),
        Err(WriteError::Mismatch { lba: victim_sector })
    );
}
