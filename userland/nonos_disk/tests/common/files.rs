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

//! An image with recognisable bytes, shaped like the shipped one and
//! scaled down.

use nonos_disk::NonosImage;

pub struct Files {
    pub boot_efi: Vec<u8>,
    pub kernel_bin: Vec<u8>,
    pub boot_cfg: Vec<u8>,
    pub boot_trailer: Vec<u8>,
    pub boot_root: Vec<u8>,
    pub kernel_approval: Vec<u8>,
}

pub fn files() -> Files {
    let pattern = |len: usize, seed: u32| -> Vec<u8> {
        (0..len).map(|i| (i as u32).wrapping_mul(2654435761).wrapping_add(seed) as u8).collect()
    };
    Files {
        boot_efi: pattern(1_400_000 + 7, 1),
        kernel_bin: pattern(8_500_000 + 333, 2),
        boot_cfg: b"timeout=0\ndefault=nonos\n".to_vec(),
        boot_trailer: pattern(70_000 + 11, 3),
        boot_root: pattern(186, 4),
        kernel_approval: pattern(128, 5),
    }
}

pub fn image(f: &Files) -> NonosImage<'_> {
    NonosImage {
        boot_efi: &f.boot_efi,
        kernel_bin: &f.kernel_bin,
        boot_cfg: &f.boot_cfg,
        boot_trailer: &f.boot_trailer,
        boot_root: &f.boot_root,
        kernel_approval: Some(&f.kernel_approval),
    }
}
