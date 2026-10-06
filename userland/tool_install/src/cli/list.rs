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

//! The disk list, one line each: the word, the part, the size, the bus,
//! what it holds. A driver that did not answer is listed with its fault.

use nonos_blk_client::{survey, Disk};

use super::source::bytes;

pub fn run() -> i32 {
    let found = survey();
    let disks = &found.disks;
    if found.raid_hides_disks() {
        println!("Intel RST/VMD is on: set the BIOS storage mode to AHCI (or turn VMD off), then boot this stick again");
    }
    if disks.is_empty() {
        println!("no block driver is serving a disk on this boot");
        return 2;
    }
    println!("{:<8} {:<28} {:>10}  {:<12} holds", "word", "disk", "size", "bus");
    for d in disks {
        println!("{}", line(d));
    }
    println!();
    println!("install write <word> erases that disk and installs the running image.");
    0
}

pub fn line(d: &Disk) -> String {
    match &d.fault {
        Some(fault) => {
            let name = d.identity.map(|i| i.model_str().to_string());
            let size = if d.bytes() > 0 { bytes(d.bytes()) } else { String::from("-") };
            let name = name.as_deref().unwrap_or(d.label());
            format!("{:<8} {:<28} {:>10}  {:<12} {}", "-", name, size, d.label(), fault)
        }
        None => {
            let name = d.identity.map(|i| i.model_str().to_string());
            let name = name.as_deref().unwrap_or(d.label());
            format!(
                "{:<8} {:<28} {:>10}  {:<12} {}",
                d.confirm_word(),
                name,
                bytes(d.bytes()),
                d.label(),
                d.contents.text()
            )
        }
    }
}
