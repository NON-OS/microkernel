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

//! Dropping a file from the store, and what a boot finds when the power
//! fails part way through.
//!
//! The remover used to shift every descriptor after the removed one down a
//! slot and write the table back, header last. A descriptor whose name and
//! extent sit in two sectors then had one half shifted and the other not:
//! one file's name over another file's extent and digest, which verifies,
//! so the next boot served the wallet's name with some other file's bytes.

use std::collections::BTreeSet;

use nonos_libc::disk;

use super::blk::store_remove::remove;
use super::fixture::{bytes, image, install, refs, Layout};
use super::run::{digests, every_cut, load_counting};

fn files(n: usize) -> Vec<(String, Vec<u8>)> {
    (0..n).map(|i| (format!("/kept/{i}"), bytes(i as u8, 100 + i * 211))).collect()
}

type Seen = BTreeSet<(String, usize, [u8; 16])>;

fn set(files: &[(String, Vec<u8>)]) -> Seen {
    digests(files).into_iter().collect()
}

/// Every file the boot finds is one the store held, under its own name and
/// with its own bytes. Either all the files but `gone` are there, with or
/// without `gone`, and nothing was left out; or one descriptor was left out
/// as damaged, which is `gone`'s, and the rest are there.
fn sound(old: &Seen, gone: &str, got: Seen, refused: usize) -> Result<(), String> {
    if let Some(stray) = got.iter().find(|f| !old.contains(f)) {
        return Err(format!("{stray:?} is not a file the store held"));
    }
    let rest: Seen = old.iter().filter(|f| f.0 != gone).cloned().collect();
    match refused {
        0 if got == *old || got == rest => Ok(()),
        1 if got == rest => Ok(()),
        _ => Err(format!(
            "{refused} left out, found {:?}",
            got.iter().map(|f| &f.0).collect::<Vec<_>>()
        )),
    }
}

#[test]
fn a_removal_cut_at_any_sector_never_serves_one_name_with_another_files_bytes() {
    for n in [1usize, 3, 4, 5, 8, 9, 13, 40, 127] {
        let all = files(n);
        let old = set(&all);
        let victims: BTreeSet<usize> = [0, 1, 2, 3, 7, n / 2, n.saturating_sub(2), n - 1]
            .into_iter()
            .filter(|&k| k < n)
            .collect();
        for k in victims {
            install(&image(&refs(&all), Layout::Installer));
            let gone = all[k].0.clone();
            every_cut(&|| _ = remove(&gone), &|cut, got| {
                let (got, refused) = got.expect("the store still loads");
                let got: Seen = got.into_iter().collect();
                if let Err(why) = sound(&old, &gone, got, refused) {
                    panic!("{n} files, removing {k}, cut at {cut}: {why}");
                }
            });
            let (got, refused) = load_counting().expect("loads");
            assert_eq!((set(&got), refused), (set(&super::fixture::without(&all, k)), 0));
        }
    }
}

#[test]
fn a_name_the_table_holds_twice_is_removed_twice() {
    let mut all = files(6);
    all[4].0 = all[1].0.clone();
    install(&image(&refs(&all), Layout::Installer));
    assert_eq!(remove(&all[1].0), Ok(()));
    let (got, refused) = load_counting().expect("loads");
    assert!(got.iter().all(|(n, _)| *n != all[1].0) && refused == 0);
    assert_eq!(got.len(), 4);
}

#[test]
fn removing_a_name_the_table_does_not_hold_writes_nothing() {
    install(&image(&refs(&files(5)), Layout::Packer));
    assert_eq!(remove("/not/there"), Ok(()));
    assert_eq!(disk::landed(), 0);
}

/*
 * A removal leaves the last descriptor's old bytes past the new count; the
 * next keep takes that slot, and none of the longer name it held shows
 * through the shorter one.
 */
#[test]
fn a_keep_after_a_removal_takes_the_freed_slot_cleanly() {
    let mut all = files(6);
    all[5].0 = String::from("/a/rather/long/name/the/next/keep/lacks");
    install(&image(&refs(&all), Layout::Installer));
    assert_eq!(remove(&all[2].0), Ok(()));
    assert_eq!(super::blk::store_write::append("/b", b"short"), Ok(()));
    let mut want = super::fixture::without(&all, 2);
    want.push((String::from("/b"), b"short".to_vec()));
    let (got, refused) = load_counting().expect("loads");
    assert_eq!((set(&got), refused), (set(&want), 0));
}
