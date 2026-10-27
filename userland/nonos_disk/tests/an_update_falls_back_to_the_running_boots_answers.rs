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

//! The disk's own answers go only when they are there and check: a disk
//! with no answers, with answers whose bytes no longer match their digest,
//! or with answers setup would not write gives way to the running boot's,
//! as an install carries them; put back as they were, the disk's go again.

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

use nonos_disk::{gather, BlockSink, DiskAnswers, StoreBuilder, StoreImage, MIN_DISK_SECTORS};
use nonos_disk_map::{STORE_BASE_LBA, TOC_SPAN};
use nonos_policy_proto::setup_record::DONE;
use setup_answers::{answers, vfs_with, ANSWERS};

#[test]
fn without_answers_that_check_on_the_disk_the_running_boots_go() {
    let (disk_store, mut stick) =
        (gather(&mut vfs_with(&answers(-5))).store, vfs_with(&answers(9)));
    let (mut disk, _) = keep_disk::installed(MIN_DISK_SECTORS, disk_store, 2);
    /* The answers are the store's first payload, past a full table. */
    let first_payload = STORE_BASE_LBA + (TOC_SPAN / 512) as u64;
    let kept = disk.read_sectors(first_payload, 1);
    /* A time zone setup offers, so the record checks and the digest does not. */
    let mut changed = kept.clone();
    changed[5] = 3;
    for (payload, from_disk) in [(&changed, false), (&kept, true)] {
        disk.write_at(first_payload, payload).unwrap();
        let mut src = DiskAnswers::new(&mut disk, &mut stick);
        assert_eq!(src.from_disk(), from_disk, "time zone byte {}", payload[5]);
        let c = gather(&mut src);
        assert_eq!(c.answers.map(|a| a.timezone), Some(if from_disk { -5 } else { 9 }));
    }
    /* A record setup would not write, under a digest that matches it. */
    let mut never = StoreBuilder::new();
    never.add_all(&[(ANSWERS, b"NSA1\0\x7f\0"), ("/nonos/setup/done", &DONE)]).unwrap();
    for store in [never.finish(), StoreImage::empty()] {
        let (mut other, _) = keep_disk::installed(MIN_DISK_SECTORS, store, 2);
        let mut src = DiskAnswers::new(&mut other, &mut stick);
        assert!(!src.from_disk(), "no answers that check on the disk");
        assert_eq!(gather(&mut src).answers.map(|a| a.timezone), Some(9));
    }
}
