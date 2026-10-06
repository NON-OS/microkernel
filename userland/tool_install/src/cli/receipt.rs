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

//! The receipt, in the form the serial log keeps: what was written and read
//! back, the identifiers a firmware menu or a partition tool shows for the
//! disk and each partition, where each partition lies, and what the store
//! and the boot partition hold.

use nonos_disk::{Receipt, Region};

use super::source::bytes;

pub fn print_receipt(r: &Receipt<'_>, verified: u64) {
    println!("[INSTALL] written {} verified {}", bytes(r.bytes_written), bytes(verified));
    println!("[INSTALL] disk {}", text(&r.disk_guid.text()));
    for (region, guid) in Region::ALL.into_iter().zip(r.partitions) {
        let x = r.layout.extent(region);
        let (id, size) = (guid.text(), bytes(x.bytes()));
        println!("[INSTALL] {} {} from LBA {}, {size}", region.what(), text(&id), x.first);
    }
    println!("[INSTALL] store {} files", r.store_files);
    println!(
        "[INSTALL] fat32 {} sectors per cluster, {} clusters",
        r.geometry.sectors_per_cluster, r.geometry.data_clusters
    );
    println!("NONOS is on the disk and every sector written read back as written.");
    println!("Remove the stick and restart to boot from it.");
}

fn text(guid: &[u8; 36]) -> &str {
    core::str::from_utf8(guid).unwrap_or("")
}
