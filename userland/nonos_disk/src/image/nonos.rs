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

//! The bootloader, the attested kernel image, the records the kernel checks
//! the loader against, the release's approval of the kernel, the loader's
//! configuration, and the script that makes a firmware shell boot it. The paths are the
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
    /// The loader's own trailer and its signed boot-root record. The kernel
    /// checks the loader against both before it trusts anything else, and a
    /// disk without them halts at "The bootloader could not be checked".
    pub boot_trailer: &'a [u8],
    pub boot_root: &'a [u8],
    /// The release's approval of this kernel, when the running image has
    /// one: the TPM releases the device secret under it, and without it the
    /// secret stays sealed.
    pub kernel_approval: Option<&'a [u8]>,
}

/// What a UEFI shell runs when dropped on the volume.
pub const STARTUP_NSH: &[u8] = b"\\EFI\\BOOT\\BOOTX64.EFI\r\n";

impl<'a> NonosImage<'a> {
    pub fn tree(&self) -> Vec<Node<'a>> {
        let boot = Node::dir("BOOT", alloc::vec![Node::file("BOOTX64.EFI", self.boot_efi)]);
        let mut files = alloc::vec![
            Node::file("kernel.bin", self.kernel_bin),
            Node::file("bootloader.trailer", self.boot_trailer),
            Node::file("boot_root.approval", self.boot_root),
            Node::file("boot.cfg", self.boot_cfg),
        ];
        if let Some(approval) = self.kernel_approval {
            files.push(Node::file("kernel.approval", approval));
        }
        let nonos = Node::dir("nonos", files);
        alloc::vec![
            Node::dir("EFI", alloc::vec![boot, nonos]),
            Node::file("startup.nsh", STARTUP_NSH),
        ]
    }

    pub fn total_bytes(&self) -> u64 {
        let approval = self.kernel_approval.map_or(0, <[u8]>::len);
        (self.boot_efi.len()
            + self.kernel_bin.len()
            + self.boot_trailer.len()
            + self.boot_root.len()
            + approval
            + self.boot_cfg.len()
            + STARTUP_NSH.len()) as u64
    }
}
