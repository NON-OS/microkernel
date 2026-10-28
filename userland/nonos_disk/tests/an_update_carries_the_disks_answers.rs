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

//! An update carries setup's answers from the disk it updates, read out of
//! the disk's own store as vfs reads it, so the system goes on with the
//! answers it was set up with and not the ones given to the stick, and the
//! new store holds them with the marker that says they were kept.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/fake_vfs.rs"]
mod fake_vfs;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/setup_answers.rs"]
mod setup_answers;
#[path = "common/vfs/mod.rs"]
mod vfs;

/*
 * The vfs files name `alloc`, as a no_std capsule's do.
 */
extern crate alloc;

use nonos_disk::{gather, update, DiskAnswers, MIN_DISK_SECTORS};
use nonos_disk_map::STORE_BASE_LBA;
use nonos_policy_proto::setup_record::DONE;
use setup_answers::{answers, vfs_with, ANSWERS};

#[test]
fn an_update_carries_the_answers_on_the_disk_not_the_sticks() {
    let (disk_store, mut stick) =
        (gather(&mut vfs_with(&answers(-5))).store, vfs_with(&answers(9)));
    let (mut disk, found) = keep_disk::installed(MIN_DISK_SECTORS, disk_store, 2);
    let mut src = DiskAnswers::new(&mut disk, &mut stick);
    assert!(src.from_disk());
    let c = gather(&mut src);
    assert_eq!(c.answers.map(|a| a.timezone), Some(-5));
    let f = files::files();
    let image = files::image(&f);
    update(&mut disk, &found, &image, c.store.clone(), entropy::ENTROPY, &mut |_| {}).unwrap();
    let loaded = vfs::load(&disk.read_sectors(STORE_BASE_LBA, c.store.bytes().len() / 512));
    let want = [(ANSWERS, answers(-5)), ("/nonos/setup/done", DONE.to_vec())];
    assert_eq!(loaded, want.map(|(n, d)| (n.to_string(), d)));
}
