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

//! The client's per-driver table against each driver's own constants. It
//! was a hand copy kept in step by convention: a renumbered op would have
//! the client send one request where another was meant, and nothing failed
//! at build or test time. A row for a driver this crate mounts no source
//! for is held by scripts/check_blk_wire.py, which finds every driver the
//! table names by its service name.

use crate::driver::{Driver, ALL};
use crate::drivers::{ahci, nvme, virtio_blk};
use crate::identity_parse::OP_AHCI_IDENTIFY;
use crate::wire::{HDR_LEN, VERSION};

fn holds(driver: Driver, magic: u32, header: (u16, usize), ops: [u16; 4]) {
    let row = driver.ops();
    assert_eq!(driver.magic(), magic, "{driver:?}: magic");
    assert_eq!((VERSION, HDR_LEN), header, "{driver:?}: version and header length");
    assert_eq!([row.capacity, row.read, row.write, row.flush], ops, "{driver:?}: ops");
}

#[test]
fn the_nvme_row_is_the_nvme_drivers_own() {
    let (h, o) = (nvme::header::MAGIC, (nvme::header::VERSION, nvme::header::HDR_LEN));
    let ops = [nvme::ops::OP_CAPACITY, nvme::ops::OP_READ_BLOCKS];
    holds(Driver::Nvme, h, o, [ops[0], ops[1], nvme::ops::OP_WRITE_BLOCKS, nvme::ops::OP_FLUSH]);
    assert_eq!(Driver::Nvme.service(0), Some(nvme::endpoint::SERVICE_NAME.as_bytes()));
}

#[test]
fn the_sata_row_is_the_ahci_drivers_own() {
    let (h, o) = (ahci::header::MAGIC, (ahci::header::VERSION, ahci::header::HDR_LEN));
    let ops = [ahci::ops::OP_CAPACITY, ahci::ops::OP_READ_BLOCKS];
    holds(Driver::Ahci, h, o, [ops[0], ops[1], ahci::ops::OP_WRITE_BLOCKS, ahci::ops::OP_FLUSH]);
    assert_eq!(Driver::Ahci.service(0), Some(ahci::endpoint::SERVICE_NAME.as_bytes()));
    assert_eq!(OP_AHCI_IDENTIFY, ahci::ops::OP_IDENTIFY, "the identify op the client sends");
}

#[test]
fn the_virtio_row_is_the_virtio_blk_drivers_own() {
    use virtio_blk::{header, ops};
    let (h, o) = (header::MAGIC, (header::VERSION, header::HDR_LEN));
    let w = [ops::OP_CAPACITY, ops::OP_READ_BLOCKS, ops::OP_WRITE_BLOCKS, ops::OP_FLUSH];
    holds(Driver::VirtioBlk, h, o, w);
}

/// The block wire's drivers share one dialect, its magic and ops alike.
#[test]
fn no_two_drivers_share_a_service_name_or_a_magic_off_the_block_wire() {
    let wire = [Driver::Wire, Driver::Ufs, Driver::Ide, Driver::Scsi];
    let ops = |d: Driver| (d.ops().capacity, d.ops().read, d.ops().write, d.ops().flush);
    for (i, &a) in ALL.iter().enumerate() {
        for &b in &ALL[i + 1..] {
            assert_ne!(a.service(0), b.service(0), "{a:?} and {b:?}");
            let one_dialect = wire.contains(&a) && wire.contains(&b);
            assert_eq!(a.magic() == b.magic(), one_dialect, "{a:?} and {b:?}: the magic");
            assert!(!one_dialect || ops(a) == ops(b), "{a:?} and {b:?}: the ops");
        }
    }
}
