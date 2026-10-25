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

//! load_by_name's reads of the four artifacts, and the cap they are read under.

use crate::alloc_watch::held_after;
use crate::fixtures::{path, put_artifacts, EXTS, MAX_ARTIFACT, NAME};
use crate::frame::{by_name, by_name_payload, einval, frame, reply};
use crate::process::INSTALLER_PID;
use crate::protocol::OP_LOAD_BY_NAME;
use crate::{clients::vfs, syscall};

fn load(sender: u32) -> Vec<u8> {
    syscall::reset(3);
    by_name(&frame(OP_LOAD_BY_NAME, &by_name_payload(0x18, &[])), sender)
}

#[test]
fn the_four_artifacts_are_read_as_the_installer_one_byte_past_max_artifact() {
    put_artifacts(NAME, None);
    load(77);
    let cap = MAX_ARTIFACT + 1;
    let want: Vec<_> = EXTS.iter().map(|e| (INSTALLER_PID, path(NAME, e), cap)).collect();
    assert_eq!(vfs::reads(), want);
}

#[test]
fn any_one_artifact_missing_is_refused() {
    for skip in 0..4 {
        put_artifacts(NAME, Some(skip));
        assert_eq!(load(77), einval());
        assert!(syscall::take().is_empty(), "loaded without artifact {skip}");
    }
}

/// The elf at n bytes, the rest as put_artifacts leaves them; the elf_len the load carries.
fn elf_of(n: usize) -> u32 {
    put_artifacts(NAME, None);
    vfs::put(&path(NAME, EXTS[0]), vec![0xE1; n]);
    assert_eq!(load(77), reply(0, &3u32.to_le_bytes()));
    let loads = syscall::take();
    assert_eq!(loads[0].blobs[0], vec![0xE1; n]);
    loads[0].req.elf_len
}

#[test]
fn an_artifact_of_max_artifact_bytes_is_taken_whole() {
    assert_eq!(elf_of(MAX_ARTIFACT as usize), MAX_ARTIFACT);
}

// read_file stops at the max it is given, so the installer asks for one byte
// more than it takes: a longer artifact is refused here, and never reaches the
// load cut to MAX_ARTIFACT.
#[test]
fn an_artifact_past_max_artifact_is_refused_and_dropped_before_the_reply() {
    put_artifacts(NAME, None);
    vfs::put(&path(NAME, EXTS[0]), vec![0xE1; MAX_ARTIFACT as usize + 1]);
    let (answer, held) = held_after(|| load(77));
    assert_eq!(answer, einval());
    assert!(syscall::take().is_empty(), "an oversize artifact reached the load syscall");
    assert!(held < 1 << 20, "{held} bytes still held after the reply");
}
