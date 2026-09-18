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

//! The image, read out of the kernel: the bootloader as the firmware loaded
//! it and the kernel image file as the bootloader verified it. Both are
//! copied once into memory this capsule owns for the life of the install,
//! and lent to the disk writer from there, which is why they are leaked
//! rather than boxed: the writer's borrows outlive every frame.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_disk::NonosImage;
use nonos_libc::{
    mk_install_source, mk_install_source_size, INSTALL_SOURCE_KERNEL_IMAGE,
    INSTALL_SOURCE_LOADER_IMAGE,
};

/// The loader's configuration as `nonos-mk-esp` writes it. Two lines, and
/// the same two lines on every image the build has ever produced; the
/// kernel does not carry the file, so the installer spells it.
pub const BOOT_CFG: &[u8] = b"timeout=0\ndefault=nonos\n";

const CHUNK: usize = 1 << 20;

pub struct Image {
    pub loader: &'static [u8],
    pub kernel: &'static [u8],
}

impl Image {
    pub fn load() -> Result<Image, String> {
        let loader = read_all(INSTALL_SOURCE_LOADER_IMAGE)?;
        let kernel = read_all(INSTALL_SOURCE_KERNEL_IMAGE)?;
        if loader.is_empty() || kernel.is_empty() {
            return Err(String::from("the bootloader did not record the running image"));
        }
        Ok(Image { loader: Vec::leak(loader), kernel: Vec::leak(kernel) })
    }

    pub fn as_disk_image(&self) -> NonosImage<'static> {
        NonosImage { boot_efi: self.loader, kernel_bin: self.kernel, boot_cfg: BOOT_CFG }
    }
}

/// Sized once from the kernel's answer: a vector that doubles as it grows
/// would hold the image twice over at the last step.
fn read_all(kind: u64) -> Result<Vec<u8>, String> {
    let size = mk_install_source_size(kind).unwrap_or(0) as usize;
    let mut out: Vec<u8> = Vec::with_capacity(size);
    let mut buf = alloc::vec![0u8; CHUNK];
    loop {
        let n = mk_install_source(kind, out.len() as u64, &mut buf);
        if n < 0 {
            return Err(alloc::format!("the kernel refused the image (errno {n})"));
        }
        if n == 0 {
            return Ok(out);
        }
        out.extend_from_slice(&buf[..n as usize]);
    }
}
