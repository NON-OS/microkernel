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

//! Every field of the container, one bit at a time and cut short, through
//! the boot load. Each outcome is one of three: the store refused whole
//! (only for the header, which is all the container checks of itself), the
//! damaged entry left out and counted, or, for the two fields no check
//! covers, a difference the format cannot see, named here so nobody takes
//! it for a proof that it can.

use nonos_disk_map::STORE_BASE_LBA;
use nonos_libc::disk;

use super::blk::error::BlkError;
use super::fixture::{field, image, install, refs, three, Layout, ENTRY_LEN, HEADER_LEN, NAME_LEN};
use super::run::load_counting;

type Files = Vec<(String, Vec<u8>)>;

/// The boot load of `img`, with the original files it served unchanged
/// split from any it served otherwise.
fn boot(img: &[u8]) -> Result<(Files, Files, usize), BlkError> {
    install(img);
    let (got, refused) = load_counting()?;
    let original = three();
    let (same, other) = got.into_iter().partition(|f| original.contains(f));
    Ok((same, other, refused))
}

fn flip(img: &[u8], byte: usize, bit: u8) -> Vec<u8> {
    let mut out = img.to_vec();
    out[byte] ^= 1 << bit;
    out
}

#[test]
fn a_flipped_magic_or_version_bit_refuses_the_store_whole() {
    let img = image(&refs(&three()), Layout::Installer);
    for byte in 0..12 {
        for bit in 0..8 {
            assert_eq!(boot(&flip(&img, byte, bit)).err(), Some(BlkError::BadContainer));
        }
    }
}

/*
 * The count is the one header field a flip can leave plausible. Past
 * MAX_ENTRIES the store is refused; above the real count the slots it
 * invents are zeros and each is left out; below it, the dropped tail is a
 * smaller store as far as the format can tell. That last case is the
 * container's own limit: the table carries no check over itself.
 */
#[test]
fn a_flipped_count_bit_refuses_invents_nothing_or_drops_the_tail_unseen() {
    let files = three();
    let img = image(&refs(&files), Layout::Installer);
    for bit in 0..32 {
        let count = 3u32 ^ (1 << bit);
        let out = flip(&img, 12 + bit / 8, (bit % 8) as u8);
        match boot(&out) {
            Err(e) => assert!(count > 128 && e == BlkError::BadContainer, "count {count}: {e:?}"),
            Ok((same, other, refused)) => {
                assert!(other.is_empty(), "count {count} served a changed file");
                if count > 3 {
                    assert_eq!((same.len(), refused), (3, count as usize - 3), "count {count}");
                } else {
                    assert_eq!((same, refused), (files[..count as usize].to_vec(), 0));
                }
            }
        }
    }
}

/*
 * The offset, length and digest of each descriptor: a flipped bit leaves
 * that one entry out (the extent leaves the window or the sector grid, or
 * the bytes stop matching the digest) and the other two load as they were.
 * The one flip that changes nothing served is in the offset of the empty
 * file, which reads no bytes wherever it points.
 */
#[test]
fn a_flipped_extent_or_digest_bit_leaves_that_entry_out_alone() {
    let files = three();
    let img = image(&refs(&files), Layout::Installer);
    for i in 0..files.len() {
        for at in NAME_LEN..ENTRY_LEN {
            for bit in 0..8 {
                let (same, other, refused) = boot(&flip(&img, field(i, at), bit)).expect("loads");
                let what = format!("entry {i} byte {at} bit {bit}");
                assert!(other.is_empty(), "{what}: changed bytes served");
                if same.len() == files.len() {
                    let harmless = files[i].1.is_empty() && at < NAME_LEN + 8;
                    assert!(harmless && refused == 0, "{what}: not caught");
                } else {
                    assert_eq!((same.len(), refused), (2, 1), "{what}");
                }
            }
        }
    }
}

/*
 * A name bit: a name that stops being a path the vfs writes is left out;
 * one that stays a valid path serves the right bytes under another name,
 * which nothing in the format can catch (the digest covers the payload,
 * not the name). Either way no file is served with another file's bytes.
 */
#[test]
fn a_flipped_name_bit_leaves_the_entry_out_or_renames_it_with_its_own_bytes() {
    let files = three();
    let img = image(&refs(&files), Layout::Installer);
    for (i, (_, bytes)) in files.iter().enumerate() {
        for at in 0..NAME_LEN {
            for bit in 0..8 {
                let (same, other, refused) = boot(&flip(&img, field(i, at), bit)).expect("loads");
                assert_eq!(same.len() + other.len() + refused, 3);
                for (name, data) in &other {
                    assert_eq!(data, bytes, "{name} serves bytes that are not its own");
                }
            }
        }
    }
}

/*
 * The store cut short at every sector: everything past the cut reads as
 * zeros, as on a disk whose write stopped there. A header that is gone is
 * the store refused; a table that is gone is its slots left out; a payload
 * that is gone fails its digest. Nothing is served wrong.
 */
#[test]
fn a_store_cut_short_at_any_sector_refuses_or_leaves_out_and_never_serves_wrong() {
    let files = three();
    let img = image(&refs(&files), Layout::Packer);
    for keep in 0..=img.len() / 512 {
        disk::reset();
        disk::put(STORE_BASE_LBA, &img[..keep * 512]);
        let Ok((got, refused)) = load_counting() else {
            assert_eq!(keep, 0, "only a missing header refuses the store");
            continue;
        };
        assert!(got.iter().all(|f| files.contains(f)), "cut at {keep}: a changed file served");
        assert_eq!(got.len() + refused, 3, "cut at {keep}");
    }
}

/*
 * The header and table read short: the decode is handed every length from
 * nothing to the whole table, and refuses whole only below the bytes the
 * count needs.
 */
#[test]
fn a_table_shorter_than_its_count_is_refused_whole_and_never_read_past() {
    use crate::vfs_blk::store_patch::store_header::entry_count;
    use crate::vfs_blk::store_patch::store_toc::{decode, Window};
    let img = image(&refs(&three()), Layout::Installer);
    let window = Window { base: STORE_BASE_LBA * 512, end: 1 << 26 };
    let need = HEADER_LEN + ENTRY_LEN * 3;
    for len in 0..=need + 1 {
        let head = entry_count(&img[..len.min(img.len())]);
        assert_eq!(head.is_ok(), len >= HEADER_LEN, "{len}");
        let toc = decode(&img[..len], 3, window);
        assert_eq!(toc.is_ok(), len >= need, "{len}");
    }
}
