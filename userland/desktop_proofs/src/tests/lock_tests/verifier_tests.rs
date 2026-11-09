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

//! What the shell knows of the verifier, and the blob it keeps on disk.

use crate::shell_lock::{Known, Lock, Verifier};

#[test]
fn nothing_is_offered_until_the_keyring_says_a_passphrase_is_set() {
    let mut l = Lock::default();
    assert!(!l.offered());
    l.verifier.standing(false, &[]);
    assert!(!l.offered());
    assert!(l.verifier.ask_due(), "none set: asked once this boot");
    l.verifier.asked();
    assert!(!l.verifier.ask_due());
    l.verifier.standing(true, &[1, 2, 3]);
    assert!(l.offered());
    assert!(!l.verifier.ask_due());
}

#[test]
fn each_new_blob_is_written_once() {
    let mut v = Verifier::default();
    v.took(&[1, 2, 3]);
    assert_eq!(v.due_write(), Some(&[1u8, 2, 3][..]));
    v.written();
    assert_eq!(v.due_write(), None);
    v.took(&[1, 2, 3]);
    assert_eq!(v.due_write(), None, "the same blob again is not written again");
    v.took(&[4]);
    assert_eq!(v.due_write(), Some(&[4u8][..]));
}

#[test]
fn a_blob_read_from_disk_is_kept_already_and_an_empty_one_never_written() {
    let mut v = Verifier::default();
    v.loaded(&[9, 9]);
    assert_eq!(v.known(), Known::Set);
    assert_eq!(v.due_write(), None);
    let mut w = Verifier::default();
    w.took(&[]);
    assert_eq!(w.known(), Known::Set);
    assert_eq!(w.due_write(), None);
}

#[test]
fn the_blob_survives_a_keyring_that_lost_its_record() {
    let mut v = Verifier::default();
    v.took(&[7, 7, 7]);
    v.standing(false, &[]);
    assert_eq!(v.known(), Known::Absent);
    assert_eq!(v.blob(), &[7, 7, 7], "kept to hand back");
}

#[test]
fn the_band_is_held_for_the_screen_or_the_prompt() {
    let mut l = Lock::default();
    assert!(!l.in_band());
    l.screen.lock();
    assert!(l.in_band());
    let p = Lock {
        choose: Some(crate::shell_lock::Choose::new(crate::shell_lock::Purpose::First)),
        ..Lock::default()
    };
    assert!(p.in_band());
}
