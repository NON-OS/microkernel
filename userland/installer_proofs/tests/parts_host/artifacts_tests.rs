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

//! Artifacts::read and Artifacts::request directly.

use crate::artifacts::Artifacts;
use crate::fixtures::{artifact, path, put_artifacts, EXTS, MAX_ARTIFACT, NAME};
use crate::syscall::{self, N_MK_CAPSULE_LOAD};
use crate::{clients::vfs, mk_capsule_load};

const INSTALLER_PID: u32 = 7;

fn elf_len_read(n: usize) -> Option<u32> {
    put_artifacts(NAME, None);
    vfs::put(&path(NAME, EXTS[0]), vec![0xE1; n]);
    Artifacts::read(INSTALLER_PID, NAME).map(|a| a.request(0, &[], 0).elf_len)
}

#[test]
fn read_takes_max_artifact_bytes_and_cuts_one_more_to_max() {
    assert_eq!(elf_len_read(MAX_ARTIFACT as usize), Some(MAX_ARTIFACT));
    assert_eq!(elf_len_read(MAX_ARTIFACT as usize + 1), Some(MAX_ARTIFACT));
}

#[test]
fn read_is_none_when_any_one_artifact_is_missing() {
    for skip in 0..4 {
        put_artifacts(NAME, Some(skip));
        assert!(Artifacts::read(INSTALLER_PID, NAME).is_none(), "without {skip}");
    }
}

#[test]
fn request_takes_on_behalf_of_from_its_argument_alone() {
    put_artifacts(NAME, None);
    let a = Artifacts::read(INSTALLER_PID, NAME).expect("all four present");
    let args = 0xDEAD_BEEFu32.to_le_bytes();
    for pid in [0, 1, 4242, u32::MAX] {
        let r = a.request(u64::from(INSTALLER_PID), &args, pid);
        assert_eq!(r.on_behalf_of, pid);
        assert_eq!((r.args_ptr, r.args_len), (args.as_ptr() as u64, 4));
        assert_eq!(r.requested_caps, u64::from(INSTALLER_PID));
    }
    let r = a.request(0, &[], 9);
    assert_eq!((r.args_ptr, r.args_len, r.on_behalf_of), (0, 0, 9));
}

#[test]
fn request_names_the_four_blobs_read_for_the_load_syscall() {
    put_artifacts(NAME, None);
    let a = Artifacts::read(INSTALLER_PID, NAME).expect("all four present");
    syscall::reset(12);
    let args = *b"run";
    assert_eq!(mk_capsule_load(&a.request(5, &args, 9)), 12);
    let loads = syscall::take();
    assert_eq!((loads.len(), loads[0].num, loads[0].req.elf_len), (1, N_MK_CAPSULE_LOAD, 4));
    for i in 0..4 {
        assert_eq!(loads[0].blobs[i], artifact(i), "artifact {i}");
    }
    assert_eq!(loads[0].blobs[4], args);
}
