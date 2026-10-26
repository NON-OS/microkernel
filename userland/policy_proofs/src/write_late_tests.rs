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

//! The keeper's write against a vfs that answers late or refuses: a write is
//! taken as kept only when the disk holds its bytes, a retried write is
//! harmless over a late one, and the disk ends on the newest snapshot.

use crate::write_late::Model;
use crate::write_steps::{write, written_after};

const DIRS: [&[u8]; 2] = [b"/nonos", b"/nonos/settings"];
const PATH: &[u8] = b"/nonos/settings/values";

/// The snapshot a change count stands for.
fn snapshot(count: u32) -> Vec<u8> {
    count.to_le_bytes().repeat(8)
}

#[test]
fn a_write_taken_as_kept_is_on_the_disk() {
    for seed in 1..20_000u64 {
        let mut vfs = Model::new(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        let bytes = snapshot(seed as u32);
        let kept = write(&mut vfs, &DIRS, PATH, &bytes).is_ok();
        if kept {
            assert_eq!(vfs.disk.get(PATH), Some(&bytes), "seed {seed}");
        }
        assert_eq!(written_after(0, 7, kept), if kept { 7 } else { 0 }, "seed {seed}");
    }
}

/* Changes keep coming while writes run late or are refused. The keeper
 * tries the newest snapshot until one is kept; the disk then holds that
 * one, and never a snapshot older than one already kept. */
#[test]
fn retries_over_late_writes_end_on_the_newest_snapshot() {
    for seed in 1..5_000u64 {
        let mut vfs = Model::new(seed.wrapping_mul(0xd1b5_4a32_d192_ed03) | 1);
        let (mut written, mut changes) = (0u32, 0u32);
        for step in 0..40u32 {
            if step % 3 == 0 {
                changes += 1;
            }
            if changes == written {
                continue;
            }
            let kept = write(&mut vfs, &DIRS, PATH, &snapshot(changes)).is_ok();
            written = written_after(written, changes, kept);
            if written > 0 {
                let on_disk = vfs.disk.get(PATH).expect("a kept record");
                let held = u32::from_le_bytes(on_disk[..4].try_into().unwrap());
                assert!(held >= written, "seed {seed}: disk {held}, kept {written}");
            }
        }
        while written != changes {
            let kept = write(&mut vfs, &DIRS, PATH, &snapshot(changes)).is_ok();
            written = written_after(written, changes, kept);
        }
        assert_eq!(vfs.disk.get(PATH), Some(&snapshot(changes)), "seed {seed}");
    }
}
