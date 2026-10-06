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

//! Keeping a file: what the store holds after an append, after one cut
//! short at any sector, and after one the store has no room for.
//!
//! The live USB stick and the QEMU disk are packed by nonos-store-pack, which
//! puts the first payload right after the table. Every fourth entry the
//! table needs one more sector, and the appender used to write it over that
//! payload: the first sector of /capsules/std_proof.elf, with no digest to
//! notice, each time a live session kept its fourth, eighth, twelfth file.

use nonos_disk_map::{STORE_END_LBA, TOC_SPAN};
use nonos_libc::disk;

use super::blk::error::BlkError;
use super::blk::store_write::append;
use super::fixture::NAME_LEN;
use super::fixture::{bytes, field, image, install, refs, set_offset, three, Layout, BASE};
use super::run::{digests, every_cut, load, load_counting};

fn with(files: &[(String, Vec<u8>)], name: &str, data: &[u8]) -> Vec<(String, Vec<u8>)> {
    let mut out = files.to_vec();
    out.push((String::from(name), data.to_vec()));
    out
}

fn loads_as(want: &[(String, Vec<u8>)]) {
    let got = load_counting().map(|(files, refused)| (digests(&files), refused));
    assert_eq!(got, Ok((digests(want), 0)));
}

#[test]
fn growing_a_packed_table_moves_the_payload_in_its_way() {
    for first_empty in [false, true] {
        let mut files = three();
        if first_empty {
            files.rotate_right(1);
        }
        install(&image(&refs(&files), Layout::Packer));
        let data = bytes(9, 700);
        assert_eq!(append("/nonos/wifi/saved", &data), Ok(()));
        loads_as(&with(&files, "/nonos/wifi/saved", &data));
    }
}

#[test]
fn every_append_to_a_packed_store_keeps_every_file() {
    let mut files = vec![(String::from("/capsules/std_proof.elf"), bytes(1, 9_000))];
    install(&image(&refs(&files), Layout::Packer));
    for i in 1..nonos_disk_map::MAX_ENTRIES {
        let (name, data) = (format!("/nonos/kept/{i}"), bytes(i as u8, 1 + (i * 37) % 900));
        assert_eq!(append(&name, &data), Ok(()), "append {i}");
        files.push((name, data));
        loads_as(&files);
    }
}

#[test]
fn an_append_cut_at_any_sector_leaves_the_old_store_or_the_new_one() {
    for layout in [Layout::Installer, Layout::Packer] {
        let files = three();
        install(&image(&refs(&files), layout));
        let data = bytes(5, 20_000);
        let (old, new) = (digests(&files), digests(&with(&files, "/nonos/new", &data)));
        every_cut(&|| _ = append("/nonos/new", &data), &|cut, got| {
            let (got, refused) = got.expect("the store still loads");
            assert_eq!(refused, 0, "{layout:?} cut at {cut}: an entry left out as damaged");
            assert!(got == old || got == new, "{layout:?} cut at {cut}: {got:?}");
        });
    }
}

/*
 * Keeping a record again under its name with new bytes of the same length:
 * the new bytes go to a free extent first and one sector repoints the
 * descriptor, so a cut anywhere serves the old bytes or the new ones.
 */
#[test]
fn a_replacement_cut_at_any_sector_serves_the_old_bytes_or_the_new() {
    for layout in [Layout::Installer, Layout::Packer] {
        let mut files = three();
        files[0] = (String::from("/nonos/wallet/vault"), bytes(1, 3000));
        install(&image(&refs(&files), layout));
        let fresh = bytes(77, 3000);
        let mut renewed = files.clone();
        renewed[0].1 = fresh.clone();
        let (old, new) = (digests(&files), digests(&renewed));
        every_cut(&|| _ = append("/nonos/wallet/vault", &fresh), &|cut, got| {
            let (got, refused) = got.expect("the store still loads");
            assert_eq!(refused, 0, "{layout:?} cut at {cut}");
            assert!(got == old || got == new, "{layout:?} cut at {cut}: {got:?}");
        });
        loads_as(&renewed);
    }
}

/*
 * One-byte files two MiB apart across the whole store window, the last in
 * the store's final sector: far under the budget, but no gap holds four MiB
 * and nothing fits past the last. The appender used to put the payload
 * after the last extent regardless, and the kernel refused the write as
 * past the store. The count follows the window, so it holds for any plan
 * position the disk map sets.
 */
#[test]
fn a_payload_no_free_extent_fits_is_no_space_and_writes_nothing() {
    let end = STORE_END_LBA * 512;
    let n = ((end - BASE) >> 21) as usize;
    let spot = |i: usize| if i == n - 1 { end - 512 } else { BASE + ((i as u64 + 1) << 21) };
    let names: Vec<String> = (0..n).map(|i| format!("/spread/{i}")).collect();
    let files: Vec<(&str, &[u8])> = names.iter().map(|n| (n.as_str(), &b"x"[..])).collect();
    let mut img = image(&files, Layout::Installer);
    for i in 0..files.len() {
        set_offset(&mut img, i, spot(i));
    }
    install(&img[..TOC_SPAN]);
    for i in 0..files.len() {
        disk::put(spot(i) / 512, b"x");
    }
    assert_eq!(load().map(|f| f.len()), Ok(n));
    assert_eq!(append("/nonos/big", &vec![3u8; 4 << 20]), Err(BlkError::NoSpace));
    assert_eq!(disk::landed(), 0);
}

#[test]
fn a_slot_past_the_count_is_cleared_before_a_new_name_goes_in() {
    let files = three();
    let mut img = image(&refs(&files), Layout::Installer);
    let stale = field(files.len(), 0);
    img[stale..stale + NAME_LEN].fill(b'z');
    img[stale] = b'/';
    install(&img);
    assert_eq!(append("/a", b"kept"), Ok(()));
    loads_as(&with(&files, "/a", b"kept"));
}

#[test]
fn a_write_the_device_took_short_fails_the_keep_and_leaves_the_store() {
    let files = three();
    install(&image(&refs(&files), Layout::Installer));
    disk::short_writes();
    assert!(matches!(append("/nonos/new", &bytes(4, 5000)), Err(BlkError::Transport(_))));
    loads_as(&files);
}
