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

//! The receipt, in the form the serial log keeps: what was written, what
//! was read back, and the identifiers a firmware menu shows for the disk.

use nonos_disk::Receipt;

use super::source::bytes;

pub fn print_receipt(r: &Receipt<'_>, verified: u64) {
    println!("[INSTALL] written {} verified {}", bytes(r.bytes_written), bytes(verified));
    println!("[INSTALL] disk {}", text(&r.disk_guid.text()));
    println!(
        "[INSTALL] partition {} ({} sectors from {})",
        text(&r.partition_guid.text()),
        r.layout.esp_sectors(),
        r.layout.esp_first_lba
    );
    println!(
        "[INSTALL] fat32 {} sectors per cluster, {} clusters",
        r.geometry.sectors_per_cluster, r.geometry.data_clusters
    );
    println!("NONOS is on the disk and every sector read back as written.");
    println!("Remove the stick and restart to boot from it.");
}

fn text(guid: &[u8; 36]) -> &str {
    core::str::from_utf8(guid).unwrap_or("")
}
