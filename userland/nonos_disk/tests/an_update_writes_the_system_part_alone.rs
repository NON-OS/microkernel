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

//! An update writes the boot partition and the store, and nothing else:
//! every sector of every write lies in NONOS-ESP or NONOS-STORE, none in
//! the protective MBR, either table, NONOS-PLAN or NONOS-DATA, and every
//! sector there reads as it did before. The read-back reads exactly the
//! sectors the update wrote. On the smallest disk and on one past 2 TiB.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/newer.rs"]
mod newer;
#[path = "common/recorder.rs"]
mod recorder;

use nonos_disk::{update, verify, StoreImage, MIN_DISK_SECTORS};
use recorder::{sectors, Recorder};

#[test]
fn an_update_writes_nothing_but_the_boot_partition_and_the_store() {
    for total in [MIN_DISK_SECTORS, (1u64 << 32) + (1 << 21) + 77] {
        let (disk, found) = keep_disk::installed(total, StoreImage::empty(), 2);
        let l = found.layout;
        let before = disk.written.clone();
        let mut rec = Recorder::new(disk);
        let (f, store) = (newer::newer(), StoreImage::empty());
        let image = files::image(&f);
        let r = update(&mut rec, &found, &image, store, entropy::ENTROPY, &mut |_| {}).unwrap();
        let written = sectors(&rec.writes);
        let stray = written.iter().find(|&&s| !l.store.contains(s) && !l.esp.contains(s));
        assert_eq!(stray, None, "{total}: a write outside the store and the boot partition");
        let kept = [(0, 34), (l.plan.first, l.plan.end()), (l.data.first, l.data.end())];
        for (first, end) in kept.into_iter().chain([(l.backup_array_lba, total)]) {
            let now: Vec<_> = rec.disk.written.range(first..end).collect();
            assert_eq!(
                now,
                before.range(first..end).collect::<Vec<_>>(),
                "{total}: {first}..{end}"
            );
        }
        assert!(before.range(l.data.first..l.data.end()).count() >= 256 + 64 + 8, "data to keep");
        rec.reads.clear();
        let checked = verify(&mut rec, &r, &mut |_| {}).unwrap();
        assert_eq!(sectors(&rec.reads), written, "{total}: the read-back reads what was written");
        assert_eq!(checked, written.len() as u64 * 512, "{total}: and reads each sector once");
    }
}
