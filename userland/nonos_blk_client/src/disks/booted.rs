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
 * What the kernel knows of the medium this machine booted from, asked
 * once per scan: the loader's record of its partition, or, when there is
 * none, the loader's length and first bytes. A disk they identify is the
 * boot media, and the scan leaves it off the list. A kernel that answers
 * neither (a capsule without the install-source grant) identifies nothing.
 */

use alloc::vec::Vec;

use nonos_disk::{is_boot_media, BootEvidence, BootPartition, BOOT_MEDIA_LEN};
use nonos_libc::install_source::{
    mk_install_source, mk_install_source_size, INSTALL_SOURCE_BOOT_MEDIA,
    INSTALL_SOURCE_LOADER_IMAGE,
};

use crate::device::BlockDevice;
use crate::sink::DeviceSink;

/* Past a PE image's headers and into its code. */
const HEAD: usize = 4096;

pub struct Booted {
    partition: Option<BootPartition>,
    loader_size: u64,
    loader_head: Vec<u8>,
}

impl Booted {
    pub fn ask() -> Booted {
        let mut record = [0u8; BOOT_MEDIA_LEN];
        let n = mk_install_source(INSTALL_SOURCE_BOOT_MEDIA, 0, &mut record);
        let partition = (n == BOOT_MEDIA_LEN as i64).then(|| BootPartition::parse(&record));
        if let Some(partition) = partition.flatten() {
            return Booted { partition: Some(partition), loader_size: 0, loader_head: Vec::new() };
        }
        let mut head = alloc::vec![0u8; HEAD];
        let n = mk_install_source(INSTALL_SOURCE_LOADER_IMAGE, 0, &mut head);
        head.truncate(n.max(0) as usize);
        let size = mk_install_source_size(INSTALL_SOURCE_LOADER_IMAGE).filter(|_| !head.is_empty());
        Booted { partition: None, loader_size: size.unwrap_or(0), loader_head: head }
    }

    pub fn is(&self, device: &BlockDevice) -> bool {
        let ev = BootEvidence {
            partition: self.partition,
            loader_size: self.loader_size,
            loader_head: &self.loader_head,
        };
        let found = is_boot_media(&mut DeviceSink { device: *device }, &ev);
        if found {
            let how = if self.partition.is_some() { "its partition" } else { "its loader" };
            let line =
                alloc::format!("[BLK] {:?} disk is the boot media by {how}\n", device.driver);
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
        }
        found
    }
}
