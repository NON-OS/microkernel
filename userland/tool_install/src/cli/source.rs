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

//! The image out of the kernel, and the two-line loader configuration the
//! build writes beside it. The same bytes the window installer writes.

use nonos_disk::NonosImage;
use nonos_libc::{
    mk_install_source, mk_install_source_size, INSTALL_SOURCE_KERNEL_IMAGE,
    INSTALL_SOURCE_LOADER_IMAGE,
};

/// The loader's configuration as `nonos-mk-esp` writes it.
pub const BOOT_CFG: &[u8] = b"timeout=0\ndefault=nonos\n";

pub struct Image {
    pub loader: Vec<u8>,
    pub kernel: Vec<u8>,
}

impl Image {
    pub fn load() -> Result<Image, String> {
        let loader = read_all(INSTALL_SOURCE_LOADER_IMAGE)?;
        let kernel = read_all(INSTALL_SOURCE_KERNEL_IMAGE)?;
        if loader.is_empty() || kernel.is_empty() {
            return Err(String::from("the bootloader did not record the running image"));
        }
        Ok(Image { loader, kernel })
    }

    pub fn as_disk_image(&self) -> NonosImage<'_> {
        NonosImage { boot_efi: &self.loader, kernel_bin: &self.kernel, boot_cfg: BOOT_CFG }
    }
}

fn read_all(kind: u64) -> Result<Vec<u8>, String> {
    let size = mk_install_source_size(kind).unwrap_or(0) as usize;
    let mut out = Vec::with_capacity(size);
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = mk_install_source(kind, out.len() as u64, &mut buf);
        if n < 0 {
            return Err(format!("the kernel refused the image (errno {n})"));
        }
        if n == 0 {
            return Ok(out);
        }
        out.extend_from_slice(&buf[..n as usize]);
    }
}

/// Decimal units, as the sticker on the drive has them.
pub fn bytes(n: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let (mut v, mut u) = (n as f64, 0usize);
    while v >= 1000.0 && u < UNITS.len() - 1 {
        v /= 1000.0;
        u += 1;
    }
    if u == 0 {
        return format!("{n} B");
    }
    format!("{:.1} {}", v, UNITS[u])
}
