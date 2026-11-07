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
//! A written file's room past its length given back when its writer closes
//! it (`capsule_vfs/src/store/fdtable/budget.rs` trim), so files written a
//! piece at a time are held to the budget by what they hold.

use super::{file, A, MIB};
use crate::vfs_store::{Store, StoreError, DATA_BYTES_MAX};

/// What capsule_linux's packs go in as: app_skeleton's vfs write_file sends
/// a file 60 KiB a write.
const PIECE: usize = 60 * 1024;

/// Write `bytes` to a new file of A's, PIECE a write; its handle, still
/// open, and the bytes written before a refusal.
fn written(s: &mut Store, path: &str, bytes: usize) -> (u32, usize) {
    let fd = file(s, path, A);
    let piece = vec![0x3Cu8; PIECE];
    let mut done = 0;
    while done < bytes {
        let Ok(n) = s.write(fd, A, &piece[..(bytes - done).min(PIECE)]) else { break };
        done += n as usize;
    }
    (fd, done)
}

#[test]
fn closing_a_written_file_gives_back_the_room_past_its_length() {
    let mut s = Store::new();
    let (fd, done) = written(&mut s, "/pack", 33 * MIB);
    assert_eq!(done, 33 * MIB);
    let open = s.held_except(None);
    assert!(open >= 45 * MIB, "doubling left {open} bytes held for 33 MiB");
    s.close(fd, A).expect("closes");
    assert_eq!(s.held_except(None), 33 * MIB);
    let fd = s.open("/pack", A, false, false, false, false).expect("opens");
    let back = s.read(fd, A, 33 * MIB).expect("reads");
    assert_eq!(back.len(), 33 * MIB);
    assert!(back.iter().all(|&b| b == 0x3C), "the bytes moved whole");
}

#[test]
fn packs_written_and_closed_fill_the_budget_with_what_they_hold() {
    // Three 50 MiB packs, each 60 MiB allocated while written: two left
    // 120 of the 160 MiB held, and the third was refused at 40 MiB.
    let mut s = Store::new();
    for i in 0..3 {
        let (fd, done) = written(&mut s, &format!("/pack{i}"), 50 * MIB);
        assert_eq!(done, 50 * MIB, "pack {i}");
        s.close(fd, A).expect("closes");
    }
    assert_eq!(s.held_except(None), 150 * MIB);
    assert!(s.held_except(None) <= DATA_BYTES_MAX);
}

#[test]
fn a_reader_closing_and_a_small_slack_change_nothing() {
    let mut s = Store::new();
    let (fd, _) = written(&mut s, "/written", 8 * MIB);
    let held = s.held_except(None);
    let reader = s.open("/written", A, false, false, false, false).expect("opens");
    s.close(reader, A).expect("closes");
    assert_eq!(s.held_except(None), held, "a handle that could not write trims nothing");
    s.close(fd, A).expect("closes");
    assert_eq!(s.held_except(None), 8 * MIB);
    // A file whose room is its length within TRIM_SLACK is not copied.
    let (fd, _) = written(&mut s, "/exact", 8 * PIECE);
    let before = s.held_except(None);
    s.close(fd, A).expect("closes");
    assert_eq!(s.held_except(None), before);
    assert_eq!(s.close(fd, A), Err(StoreError::BadFd), "a handle closes once");
}

#[test]
fn a_writer_that_ended_without_closing_gives_the_room_back_as_its_close_would() {
    let mut s = Store::new();
    let (_, done) = written(&mut s, "/pack", 33 * MIB);
    assert_eq!(done, 33 * MIB);
    assert!(s.held_except(None) >= 45 * MIB);
    assert_eq!(s.close_ended(|pid| pid != A), 1);
    assert_eq!(s.held_except(None), 33 * MIB, "the reap left the doubling's room held");
    let fd = s.open("/pack", A, false, false, false, false).expect("opens");
    let back = s.read(fd, A, 33 * MIB).expect("reads");
    assert!(back.len() == 33 * MIB && back.iter().all(|&b| b == 0x3C), "the bytes moved whole");
}
