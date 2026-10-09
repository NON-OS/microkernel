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

//! The rows both installers show before a disk is erased say what the
//! writer will do, read off the same plan the session writes: the whole
//! disk erased, the table, the store with what it carries, the plan, the
//! data volume and the ESP, with their sizes and where the kernel reads.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/fake_vfs.rs"]
mod fake_vfs;
#[path = "common/files.rs"]
mod files;

use fake_vfs::FakeVfs;
use nonos_disk::{describe, gather, size_text, Plan, MIN_DISK_SECTORS};
use nonos_disk_map::MAX_TOTAL_BYTES;
use nonos_policy_proto::setup_record::Answers;

#[test]
fn sizes_read_like_a_drive_label() {
    let cases = [(0, "0 B"), (999, "999 B"), (1000, "1.0 KB"), (1_949_999, "1.9 MB")];
    for (bytes, text) in cases.into_iter().chain([(MIN_DISK_SECTORS * 512, "2.3 GB")]) {
        assert_eq!(size_text(bytes), text);
    }
}

#[test]
fn the_rows_say_what_the_plan_writes() {
    let mut v = FakeVfs::default();
    let kept = Answers::decode(b"NSA1\x02\xfb\0").unwrap();
    v.put("/nonos/setup/answers", &kept.encode());
    /* A MiB past what the store holds, whatever vfs lets it hold. */
    v.put("/linux/bin/huge", &vec![0; MAX_TOTAL_BYTES as usize + (1 << 20)]);
    for proof in [".nonos_id_cert.bin", ".manifest.bin", ".zk_trailer.bin"] {
        v.put(&format!("/linux/bin/huge{proof}"), b"p");
    }
    let carried = gather(&mut v);
    let f = files::files();
    let image = files::image(&f);
    let plan =
        Plan::new(MIN_DISK_SECTORS, &image, carried.store.clone(), entropy::ENTROPY).unwrap();
    let rows: Vec<(&str, String)> =
        describe(&plan, &carried).into_iter().map(|r| (r.label, r.value)).collect();
    let want = [
        ("erased", "all 2.3 GB of it, whatever it holds now"),
        ("table", "GPT, 4 partitions, backup copy at the end"),
        ("store", "125.7 MB at LBA 256: 2 files, 69 B"),
        ("carried", "setup answers (UK, UTC-5), 0 signed programs"),
        ("left out", "1 program, 101.7 MB: more than the store's 100.7 MB"),
        ("disk plan", "LBA 245760, no imports; key header cleared"),
        ("data volume", "1.1 GB at LBA 262144, formatted on first boot"),
        ("boot", "1.1 GB at the end: loader 1.4 MB, kernel 8.5 MB"),
    ];
    assert_eq!(rows, want.map(|(l, v)| (l, v.to_string())));
}
