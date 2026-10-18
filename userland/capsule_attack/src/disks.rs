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

//! The disk drivers whose medium gate `medium` tries.

pub struct Disk {
    pub service: &'static [u8],
    pub magic: u32,
    pub read: u16,
    pub write: u16,
    pub read_what: &'static [u8],
    pub write_what: &'static [u8],
}

/// Each driver's contract, as `nonos_blk_client` copies it.
pub const DISKS: [Disk; 5] = [
    Disk {
        service: b"driver.nvme0",
        magic: 0x4E4E_564D,
        read: 7,
        write: 8,
        read_what: b"read a raw NVMe sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw NVMe sector without StoreWrite, MkIpcCall",
    },
    Disk {
        service: b"driver.ahci0",
        magic: 0x4E41_4843,
        read: 5,
        write: 6,
        read_what: b"read a raw SATA sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw SATA sector without StoreWrite, MkIpcCall",
    },
    Disk {
        service: b"driver.virtio_blk0",
        magic: 0x4E42_4C4B,
        read: 3,
        write: 4,
        read_what: b"read a raw virtio-blk sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw virtio-blk sector without StoreWrite, MkIpcCall",
    },
    Disk {
        service: b"driver.sdhci0",
        magic: 0x4E42_5731,
        read: 3,
        write: 4,
        read_what: b"read a raw eMMC sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw eMMC sector without StoreWrite, MkIpcCall",
    },
    Disk {
        service: b"driver.ufs0",
        magic: 0x4E42_5731,
        read: 3,
        write: 4,
        read_what: b"read a raw UFS sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw UFS sector without StoreWrite, MkIpcCall",
    },
];
