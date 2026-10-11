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

//! A disk the installer wrote, with the layout it chose and the ESP's
//! partition GUID. Mounted beside `mem_disk`, `files` and `entropy`.

use crate::mem_disk::MemDisk;
use crate::{entropy, files};
use nonos_disk::{install, Layout, StoreImage};

pub fn written(total: u64) -> (MemDisk, Layout, [u8; 16]) {
    let f = files::files();
    let mut disk = MemDisk::new(total);
    let image = files::image(&f);
    let r = install(&mut disk, &image, StoreImage::empty(), entropy::ENTROPY, &mut |_| {});
    let r = r.unwrap();
    (disk, r.layout, r.partitions[3].0)
}
