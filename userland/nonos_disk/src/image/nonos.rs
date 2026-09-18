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

//! The bootloader, the attested kernel image, the loader's configuration,
//! and the script that makes a firmware shell boot it. The paths are the
//! ones `nonos-mk-esp` lays out and the bootloader opens, so a disk written
//! here boots the way the USB image does.

use alloc::vec::Vec;

use crate::fat32::Node;

/// Borrowed from wherever the installer got them: the loader's copies of
/// the running image, in the shipped case.
#[derive(Clone, Copy)]
pub struct NonosImage<'a> {
    pub boot_efi: &'a [u8],
    pub kernel_bin: &'a [u8],
    pub boot_cfg: &'a [u8],
}

/// What a UEFI shell runs when dropped on the volume.
pub const STARTUP_NSH: &[u8] = b"\\EFI\\BOOT\\BOOTX64.EFI\r\n";

impl<'a> NonosImage<'a> {
    pub fn tree(&self) -> Vec<Node<'a>> {
        let boot = Node::dir("BOOT", alloc::vec![Node::file("BOOTX64.EFI", self.boot_efi)]);
        let nonos = Node::dir(
            "nonos",
            alloc::vec![
                Node::file("kernel.bin", self.kernel_bin),
                Node::file("boot.cfg", self.boot_cfg),
            ],
        );
        alloc::vec![
            Node::dir("EFI", alloc::vec![boot, nonos]),
            Node::file("startup.nsh", STARTUP_NSH),
        ]
    }

    pub fn total_bytes(&self) -> u64 {
        (self.boot_efi.len() + self.kernel_bin.len() + self.boot_cfg.len() + STARTUP_NSH.len())
            as u64
    }
}
