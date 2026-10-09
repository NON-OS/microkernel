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

//! Streamed store entries (`nonos_disk_map::streamed`): the wallpaper
//! collection is staged as its place on the device, none of its bytes, and
//! each read is a read of the device. Its size counts against the streamed
//! budget, not the heap budget the loaded entries share.

use nonos_disk_map::{MAX_TOTAL_BYTES, STREAMED_MAX_BYTES};

use super::blk::load::{Load, Step};
use super::blk::store::StoreEntry;
use super::blk::streamed::{read_range, Extent};
use super::fixture::{image, install, Layout};

const COLLECTION: &str = "/Wallpapers/collection";

/// The staged entries, and how many the load left out.
fn staged_counting() -> (Vec<StoreEntry>, usize) {
    let mut load = Load::begin().unwrap();
    loop {
        match load.step_for(8) {
            Step::More => {}
            Step::Done(staged, refused) => return (staged, refused),
            Step::Failed(e) => panic!("load failed: {e:?}"),
        }
    }
}

fn staged() -> Vec<StoreEntry> {
    let (staged, refused) = staged_counting();
    assert_eq!(refused, 0);
    staged
}

fn pattern(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i * 7 + i / 251) as u8).collect()
}

#[test]
fn the_collection_is_staged_as_its_place_and_no_bytes() {
    let collection = pattern(300_000);
    let files: Vec<(&str, &[u8])> = vec![("/capsules/a", b"abc"), (COLLECTION, &collection)];
    install(&image(&files, Layout::Packer));
    let staged = staged();
    assert_eq!(staged.len(), 2);
    assert_eq!((staged[0].data.as_slice(), staged[0].streamed), (&b"abc"[..], None));
    let e = staged[1].streamed.expect("the collection is streamed");
    assert_eq!((staged[1].name.as_str(), staged[1].data.len(), e.len), (COLLECTION, 0, 300_000));
}

#[test]
fn every_read_is_the_device_bytes_at_any_offset() {
    let collection = pattern(300_000);
    let files: Vec<(&str, &[u8])> = vec![(COLLECTION, &collection)];
    install(&image(&files, Layout::Installer));
    let e: Extent = staged()[0].streamed.unwrap();
    for (at, max) in [(0, 10), (1, 511), (511, 2), (4_000, 40_000), (299_990, 100), (123_457, 65_536)] {
        let got = read_range(&e, at as u64, max).unwrap();
        let end = (at + max).min(collection.len()).min(at + got.len());
        assert!(!got.is_empty(), "at {at}");
        assert_eq!(got, &collection[at..end], "at {at}, max {max}");
    }
    assert!(read_range(&e, 300_000, 10).unwrap().is_empty());
    assert!(read_range(&e, 0, 0).unwrap().is_empty());
}

#[test]
fn a_streamed_entry_does_not_spend_the_heap_budget() {
    // A full streamed budget beside a loaded entry one MiB short of its own:
    // past the heap budget together, and both stage, since only the loaded
    // one is loaded.
    let streamed = vec![9u8; STREAMED_MAX_BYTES as usize];
    let loaded = vec![7u8; MAX_TOTAL_BYTES as usize - (1 << 20)];
    assert!(streamed.len() + loaded.len() > MAX_TOTAL_BYTES as usize);
    let files: Vec<(&str, &[u8])> = vec![(COLLECTION, &streamed), ("/capsules/big", &loaded)];
    install(&image(&files, Layout::Packer));
    let names: Vec<String> = staged().into_iter().map(|e| e.name).collect();
    assert_eq!(names, [COLLECTION, "/capsules/big"]);
    // A loaded entry past the heap budget is still left out.
    let over = vec![7u8; MAX_TOTAL_BYTES as usize + 1];
    let files: Vec<(&str, &[u8])> = vec![("/capsules/over", &over), ("/capsules/a", b"abc")];
    install(&image(&files, Layout::Packer));
    let (staged, refused) = staged_counting();
    let names: Vec<String> = staged.into_iter().map(|e| e.name).collect();
    assert_eq!((names, refused), (vec![String::from("/capsules/a")], 1));
}

#[test]
fn a_streamed_entry_past_its_own_budget_is_left_out() {
    let over = vec![9u8; STREAMED_MAX_BYTES as usize + 1];
    let files: Vec<(&str, &[u8])> = vec![(COLLECTION, &over), ("/capsules/a", b"abc")];
    install(&image(&files, Layout::Packer));
    let (staged, refused) = staged_counting();
    let names: Vec<String> = staged.into_iter().map(|e| e.name).collect();
    assert_eq!((names, refused), (vec![String::from("/capsules/a")], 1));
}

/*
 * The appender and the replacer once checked a write against the sum of
 * every entry, the streamed collection included, while the boot load counts
 * that against its own budget. A standard image carried some 56 MiB of
 * loaded payload and a 12 MiB collection, 68 MiB together against 60, so
 * with the collection on the disk no file could be kept at all: not the wallet's
 * vault, not Settings, not the market catalogue. A write draws only on the
 * budget its own entry is counted against when the store next loads.
 */
mod writes {
    use nonos_disk_map::{MAX_TOTAL_BYTES, STREAMED_MAX_BYTES};

    use super::super::blk::error::BlkError;
    use super::super::blk::store_write::append;
    use super::super::fixture::{bytes, image, install, Layout};
    use super::{staged_counting, COLLECTION};

    const MIB: usize = 1 << 20;

    /// A store shaped like a standard image's: loaded payload, and the
    /// 12 MiB collection beside it.
    fn standard(loaded: usize) {
        let program = bytes(3, loaded);
        let collection = bytes(4, 12 * MIB);
        let files: Vec<(&str, &[u8])> =
            vec![("/capsules/program.elf", &program), (COLLECTION, &collection)];
        install(&image(&files, Layout::Packer));
    }

    /// Loaded payload that, with the collection, is past the loaded budget.
    const NEAR: usize = MAX_TOTAL_BYTES as usize - 8 * MIB;

    #[test]
    fn a_file_is_kept_beside_the_collection() {
        standard(NEAR);
        assert!(NEAR + 12 * MIB > MAX_TOTAL_BYTES as usize);
        let vault = bytes(7, 4096);
        assert_eq!(append("/nonos/wallet/vault", &vault), Ok(()));
        let (staged, refused) = staged_counting();
        assert_eq!(refused, 0);
        let kept = staged.iter().find(|e| e.name == "/nonos/wallet/vault").expect("kept");
        assert_eq!(kept.data, vault);
    }

    #[test]
    fn a_kept_file_is_replaced_beside_the_collection() {
        standard(NEAR);
        assert_eq!(append("/nonos/settings", &bytes(8, 512)), Ok(()));
        let newer = bytes(9, 512);
        assert_eq!(append("/nonos/settings", &newer), Ok(()));
        let (staged, refused) = staged_counting();
        assert_eq!(refused, 0);
        assert_eq!(staged.iter().find(|e| e.name == "/nonos/settings").unwrap().data, newer);
    }

    #[test]
    fn the_loaded_budget_still_holds() {
        let loaded = MAX_TOTAL_BYTES as usize - 1024;
        standard(loaded);
        assert_eq!(append("/nonos/too-big", &bytes(5, 2048)), Err(BlkError::NoSpace));
        assert_eq!(append("/nonos/fits", &bytes(6, 1024)), Ok(()));
        assert_eq!(staged_counting().1, 0);
    }

    #[test]
    fn the_streamed_budget_holds_for_a_streamed_name() {
        standard(MIB);
        let past = STREAMED_MAX_BYTES as usize - 12 * MIB + 1;
        assert!(MIB + 12 * MIB + past < MAX_TOTAL_BYTES as usize);
        assert_eq!(append("/Wallpapers/more", &vec![0u8; past]), Err(BlkError::NoSpace));
    }
}
