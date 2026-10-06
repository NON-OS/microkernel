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

//! One damaged entry costs that entry, not the store. A flipped bit in one
//! payload, an extent pointing outside the store, a name that does not
//! decode: each used to fail the whole table, and the boot came up with an
//! empty tree and every record the machine keeps gone with it.

use nonos_disk_map::{STORE_END_LBA, TOC_SPAN};

use super::blk::error::BlkError;
use super::blk::status;
use super::fixture::{image, install, offset_of, refs, set_len, set_name, set_offset, three};
use super::fixture::{without, Layout, BASE};
use super::run::{load, load_counting, shell_corrupt_codes, status_turn};

#[test]
fn a_payload_that_fails_its_digest_is_refused_alone() {
    let files = three();
    for layout in [Layout::Installer, Layout::Packer] {
        let mut img = image(&refs(&files), layout);
        let at = (offset_of(&img, 1) - BASE) as usize + 40_000;
        img[at] ^= 0x10;
        install(&img);
        assert_eq!(load(), Ok(without(&files, 1)), "{layout:?}");
    }
}

#[test]
fn an_extent_outside_the_payload_window_is_refused_alone() {
    let files = three();
    let window_end = STORE_END_LBA * 512;
    let outside = [
        0,                          // the protective MBR
        512 * 34,                   // past the GPT, below the store
        BASE,                       // the header itself
        BASE + 512,                 // inside the table the entries need
        window_end,                 // the disk plan
        window_end - 512,           // starts inside, runs past the end
        BASE + TOC_SPAN as u64 + 1, // not on a sector
        u64::MAX - 511,             // offset plus length wraps
    ];
    for bad in outside {
        let mut img = image(&refs(&files), Layout::Installer);
        set_offset(&mut img, 1, bad);
        install(&img);
        assert_eq!(load(), Ok(without(&files, 1)), "offset {bad}");
    }
}

#[test]
fn a_length_that_wraps_or_passes_the_budget_is_refused_alone() {
    let files = three();
    for bad in [u64::MAX, u64::MAX - 100, 61 << 20, 64 << 20] {
        let mut img = image(&refs(&files), Layout::Installer);
        set_len(&mut img, 1, bad);
        install(&img);
        assert_eq!(load(), Ok(without(&files, 1)), "length {bad}");
    }
}

#[test]
fn a_name_that_does_not_decode_is_refused_alone() {
    let files = three();
    let names: [&[u8]; 3] = [b"", b"/caps\xFFules/x", &[0xC3, 0x28]];
    for bad in names {
        let mut img = image(&refs(&files), Layout::Installer);
        set_name(&mut img, 0, bad);
        install(&img);
        assert_eq!(load(), Ok(without(&files, 0)), "name {bad:?}");
    }
}

#[test]
fn a_store_loaded_with_an_entry_left_out_still_reports_corruption() {
    let _turn = status_turn();
    let files = three();
    let mut img = image(&refs(&files), Layout::Installer);
    set_offset(&mut img, 2, 0);
    install(&img);
    let (staged, refused) = load_counting().expect("the rest loads");
    assert_eq!((staged.len(), refused), (2, 1));
    status::clear();
    status::record(&BlkError::Transport(-110));
    status::loaded(refused);
    let code = status::current();
    assert!(shell_corrupt_codes().contains(&code), "code {code} hides the damage");
    status::loaded(0);
    assert_eq!(status::current(), 0, "a whole load clears what came before");
}

#[test]
fn every_entry_damaged_loads_nothing_and_still_fails_no_one() {
    let files = three();
    let mut img = image(&refs(&files), Layout::Installer);
    for i in 0..files.len() {
        set_offset(&mut img, i, 1);
    }
    install(&img);
    assert_eq!(load(), Ok(Vec::new()));
}
