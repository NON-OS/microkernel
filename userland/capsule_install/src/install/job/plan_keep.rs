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

//! An update's plan: the chosen disk read again, so what the plan is laid
//! over is the disk as it is now and not as the list found it, then what
//! an update carries, with setup's answers from the disk itself
//! (`nonos_disk::DiskAnswers`), and the plan over the disk's own layout.
//! Reads only; nothing is written before the word is typed.

use alloc::string::String;

use nonos_blk_client::{BlockDevice, DeviceSink};
use nonos_disk::ENTROPY_BYTES;
use nonos_disk::{gather, read_installed, Carried, DiskAnswers, Keep, NonosImage, Plan};

use super::start::describe;
use crate::install::carry::VfsSource;

pub fn keep_plan(
    device: BlockDevice,
    image: &NonosImage<'static>,
    entropy: [u8; ENTROPY_BYTES],
) -> Result<(Carried, Plan<'static>), String> {
    let mut sink = DeviceSink { device };
    let installed = match read_installed(&mut sink).for_image(image) {
        Keep::Ready(installed) => installed,
        Keep::NeedsPrepare(_) => return Err(String::from("its data is not ready to keep")),
        Keep::No(why) => return Err(String::from(why.text())),
    };
    let carried = gather(&mut DiskAnswers::new(&mut sink, &mut VfsSource::new()));
    let plan = Plan::keep(&installed, image, carried.store.clone(), entropy)
        .map_err(|e| describe(e, None))?;
    Ok((carried, plan))
}
