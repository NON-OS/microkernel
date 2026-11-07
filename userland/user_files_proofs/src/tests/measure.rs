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

//! Numbers for the track notes, printed, not asserted: what a save writes
//! to the volume and what a commit costs as the tree grows. Run with
//! `cargo test --release measure -- --ignored --nocapture`. Host only.

use std::time::Instant;

use super::{fresh, pattern, put_file, settle, APP};
use crate::doubles::client;

#[test]
#[ignore]
fn what_a_save_and_a_commit_cost() {
    let (mut b, desk) = fresh();
    for files in [1_000usize, 10_000] {
        let start = b.objects().objects.len();
        for i in start..files {
            let (h, _) = client(&mut b, &desk, APP)
                .open(format!("Documents/f{i:05}.txt").as_bytes(), 2 | 4, 0o644)
                .unwrap();
            client(&mut b, &desk, APP).close(h).unwrap();
        }
        settle(&mut b, &desk);
        let before = (b.objects().puts, b.objects().put_bytes);
        client(&mut b, &desk, APP).mkdir(format!("Documents/d{files}").as_bytes(), 0o755).unwrap();
        settle(&mut b, &desk);
        let after = (b.objects().puts, b.objects().put_bytes);
        println!(
            "tree of {files} files: one mkdir commits {} objects, {} bytes",
            after.0 - before.0,
            after.1 - before.1
        );
    }
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, b"Documents/report.odt", &pattern(1, 10 << 20));
    let before = (b.objects().puts, b.objects().put_bytes);
    let t = Instant::now();
    put_file(&mut b, &desk, b"Documents/report.odt.tmp", &pattern(2, 10 << 20));
    client(&mut b, &desk, APP)
        .rename(b"Documents/report.odt.tmp", b"Documents/report.odt", 0)
        .unwrap();
    let took = t.elapsed();
    let after = (b.objects().puts, b.objects().put_bytes);
    println!(
        "a 10 MiB safe save: {} objects, {} bytes written, {:?} on host doubles",
        after.0 - before.0,
        after.1 - before.1,
        took
    );
}
