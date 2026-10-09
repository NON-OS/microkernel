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

//! Getting the install ready once a disk is chosen, before the word is
//! typed: what this boot carries over, then the whole plan against the
//! disk's size. Nothing touches the disk. A disk that cannot take NONOS is
//! said so here, with the size it would need or the block size it has.

use alloc::string::String;

use nonos_app_skeleton::clients::vfs;
use nonos_disk::{gather, Plan, ENTROPY_BYTES};
use nonos_libc::crypto_random;

use super::start::describe;
use crate::install::carry::VfsSource;
use crate::install::state::{Prepared, State};

pub fn prepare(state: &mut State) {
    state.prepared = Some(plan_for(state));
}

fn plan_for(state: &State) -> Result<Prepared, String> {
    let image = state.image.as_ref().ok_or_else(|| String::from("no image to write"))?;
    let disk = state.selected_disk().ok_or_else(|| String::from("no disk is chosen"))?;
    let device = disk.device.ok_or_else(|| String::from("that disk has no working driver"))?;
    if let Some(why) = disk.refusal() {
        return Err(why);
    }
    /*
     * Until the vfs has loaded this boot's store, the programs it holds are
     * not all there to carry.
     */
    if vfs::store_settled() != Ok(true) {
        return Err(String::from("this boot's store is still loading; choose the disk again"));
    }
    let carried = gather(&mut VfsSource::new());
    let mut entropy = [0u8; ENTROPY_BYTES];
    let rc = crypto_random(entropy.as_mut_ptr(), entropy.len());
    if rc < 0 {
        return Err(alloc::format!("the kernel gave no entropy for the identifiers (errno {rc})"));
    }
    let store = carried.store.clone();
    let plan = Plan::new(device.sectors, &image.as_disk_image(), store, entropy)
        .map_err(|e| describe(e, None))?;
    Ok(Prepared { device, plan, carried })
}
