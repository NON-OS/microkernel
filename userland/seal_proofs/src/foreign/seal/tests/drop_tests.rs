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
//! MkSealDrop: only the record whose app and seal number both match, and
//! after it nothing of that record can be peeked, dropped or taken again.

use super::keep_tests::{held, peek, seal};
use crate::errnos::{ERRNO_NOENT, ERRNO_PERM};
use crate::foreign::seal::sealed::len_for;
use crate::foreign::seal::sys_keep::sys_seal_drop;
use crate::foreign::seal::sys_read::sys_seal_read;
use crate::stand_in::SECRET;

#[test]
fn a_drop_takes_only_the_record_with_both_its_app_and_its_seal_number() {
    let (app, other) = (seal(0x62, 7), seal(0x63, 7));
    assert_eq!(sys_seal_drop(app.as_ptr() as u64, 8), ERRNO_NOENT, "another seal number");
    let missing = [0x64u8; 32];
    assert_eq!(sys_seal_drop(missing.as_ptr() as u64, 7), ERRNO_NOENT, "another app");
    assert_eq!((held(&app), held(&other)), (1, 1));
    assert_eq!(sys_seal_drop(app.as_ptr() as u64, 7), 0);
    assert_eq!((held(&app), held(&other)), (0, 1), "that one record, and no other");
    assert_eq!(peek(&app).0, ERRNO_NOENT, "nothing of it can be read again");
    assert_eq!(sys_seal_drop(app.as_ptr() as u64, 7), ERRNO_NOENT, "nor dropped twice");
    let mut out = vec![0u8; len_for(1)];
    let n = sys_seal_read(app.as_ptr() as u64, out.as_mut_ptr() as u64, out.len() as u64, 0);
    assert_eq!(n, ERRNO_NOENT, "nor taken");
}

#[test]
fn neither_answers_without_the_device_secret_gate() {
    let app = seal(0x65, 9);
    SECRET.with(|s| s.set(false));
    assert_eq!(peek(&app).0, ERRNO_PERM);
    assert_eq!(sys_seal_drop(app.as_ptr() as u64, 9), ERRNO_PERM);
    SECRET.with(|s| s.set(true));
    assert_eq!(held(&app), 1);
}

/// A dropped record cannot be looked at after it is gone, so the path is held
/// by its text: the witness is cleared before the record leaves, and nothing
/// skips Sealed's Drop.
#[test]
fn a_drop_clears_the_witness_it_takes() {
    let keep = include_str!("../../../../../../src/process/foreign/seal/sys_keep.rs");
    let cleared = "Some(mut s) => {\n            s.clear_witness();\n            0";
    let at = keep.find(cleared).expect("cleared");
    assert!(keep[..at].contains("sealed.remove(at)"));
    assert!(!keep.contains("forget") && !keep.contains("ManuallyDrop"));
    let wipe = include_str!("../../../../../../src/process/foreign/seal/wipe.rs");
    let dropped = "impl Drop for Sealed {\n    fn drop(&mut self) {\n        self.clear_witness();";
    assert!(wipe.contains(dropped));
}
