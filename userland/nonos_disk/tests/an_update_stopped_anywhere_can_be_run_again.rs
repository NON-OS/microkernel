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

//! An update stopped after any step leaves a disk that still reads as one
//! an update keeps, with the same layout and key, so the update can be run
//! again, and run again it finishes and reads back. Until its last write
//! the boot partition has no boot sector, so no firmware reads a volume
//! half written as one to boot; stopping is offered until then.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/fat_check/mod.rs"]
mod fat_check;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/newer.rs"]
mod newer;

use nonos_disk::Keep::Ready;
use nonos_disk::MIN_DISK_SECTORS;
use nonos_disk::{read_installed, update, verify, Plan, Progress, Session, StoreImage};

#[test]
fn an_update_stopped_anywhere_can_be_run_again() {
    let (mut disk, found) = keep_disk::installed(MIN_DISK_SECTORS, StoreImage::empty(), 2);
    let f = newer::newer();
    let image = files::image(&f);
    let plan = Plan::keep(&found, &image, StoreImage::empty(), entropy::ENTROPY).unwrap();
    let mut session = Session::new(plan);
    let esp = found.layout.esp.first;
    let (mut steps, mut half) = (0, None);
    loop {
        let progress = session.step(&mut disk, 1 << 20).unwrap();
        steps += 1;
        let stoppable = session.stoppable();
        assert_eq!(read_installed(&mut disk), Ready(found), "after step {steps}");
        let boot = disk.read_sectors(esp, 1);
        match progress {
            Progress::Writing { done, total } => {
                assert!(stoppable && boot.iter().all(|&b| b == 0), "step {steps}");
                half = half.or((done * 2 >= total).then(|| disk.written.clone()));
            }
            Progress::TableWritten => {
                assert_eq!((stoppable, &boot[510..]), (false, &[0x55, 0xAA][..]))
            }
            Progress::Done(_) => break,
        }
    }
    assert!(steps > 8, "the update took {steps} steps");
    disk.written = half.expect("a step past halfway");
    let r = update(&mut disk, &found, &image, StoreImage::empty(), entropy::ENTROPY, &mut |_| {});
    assert!(verify(&mut disk, &r.unwrap(), &mut |_| {}).is_ok(), "run again, it reads back");
    let v: fat_check::Volume = fat_check::check(&disk, esp);
    let kernel = v.files.get("/EFI/nonos/kernel.bin").map(Vec::as_slice);
    assert_eq!((kernel, v.dirs.len(), v.used > 0), (Some(image.kernel_bin), 3, true));
    assert_eq!((v.boot.total, v.boot.hidden as u64), (found.layout.esp.sectors, esp));
}
