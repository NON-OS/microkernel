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

//! What the installer writes for the kernel, read by the kernel's own
//! `parse_plan` and `parse_key_header` (`src/fs/blockfs_volume/`) built for
//! the host: the plan for a million disk sizes and each power of two to
//! 2^48 sectors, and the cleared key header the installer writes.
//! `an_old_nonos_disk_is_written_fresh.rs` shows these are the bytes that
//! land on the disk.

#[path = "common/crypto_shim.rs"]
mod crypto;
#[path = "common/kernel/mod.rs"]
mod kernel;

use kernel::{keyed_by, plan_of, KeyedBy, PlanError, KERNEL_AAD_END, KERNEL_KEY_LBA};
use nonos_disk::{plan_sector, Layout, MIN_DISK_SECTORS};
use nonos_disk_map::{KEY_LBA, PLAN_LBA};

fn key_header(way: u8) -> [u8; 512] {
    let mut s = [0u8; 512];
    s[..8].copy_from_slice(b"NONOSDK1");
    s[8] = way;
    s
}

#[test]
fn the_kernel_takes_the_plan_for_every_disk_size() {
    let powers = (23..=48).map(|k| 1u64 << k);
    for total in (MIN_DISK_SECTORS..MIN_DISK_SECTORS + (1 << 20)).chain(powers) {
        let l = Layout::plan(total, 50_000).unwrap();
        let sector: [u8; 512] = plan_sector(&l.data).try_into().unwrap();
        assert_eq!(plan_of(&sector, total), Ok((l.data.first, l.data.sectors, 0)), "{total}");
        assert_eq!(plan_of(&sector, l.data.end() - 1), Err(PlanError::PastEnd));
    }
}

#[test]
fn the_kernel_reads_a_cleared_key_header_as_none_so_the_tpm_keys_the_volume() {
    assert_eq!((KERNEL_KEY_LBA, KEY_LBA, KERNEL_AAD_END), (PLAN_LBA + 1, PLAN_LBA + 1, 56));
    assert_eq!(keyed_by(&[0u8; 512]), KeyedBy::NoHeader);
    assert_eq!(keyed_by(&key_header(1)), KeyedBy::Tpm);
    let mut s = key_header(2);
    s[12] = 7;
    s[24..56].fill(0x11);
    let KeyedBy::Passphrase { cost, salt, nonce, sealed } = keyed_by(&s) else { panic!() };
    assert_eq!((cost, salt, nonce, sealed), ([7, 0, 0], [0x11; 32], [0; 12], [0; 48]));
    let KeyedBy::Unknown(way) = keyed_by(&key_header(9)) else { panic!() };
    assert_eq!(way, 9);
}
