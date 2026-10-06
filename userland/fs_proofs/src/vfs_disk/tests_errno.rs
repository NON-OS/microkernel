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

//! What a keep or a removal answers, through the real appender and remover
//! and the handlers' errno map: no disk, a damaged store, no room and a bad
//! name are four answers, and a refusal writes nothing. A store read from the
//! loader's copy with no disk to write it to is no disk too, not a fault.

use nonos_disk_map::{MAX_ENTRIES, MAX_TOTAL_BYTES};
use nonos_libc::disk;

use super::blk::error::BlkError;
use super::blk::{store_remove::remove, store_write::append};
use super::fixture::{image, install, refs, set_len, three, Layout};
use super::run::errno;

const ENODEV: i32 = -19;
const EUCLEAN: i32 = -117;
const ENOSPC: i32 = -28;
const EINVAL: i32 = -22;

#[test]
fn with_no_disk_a_keep_and_a_removal_answer_no_device() {
    disk::reset();
    disk::remove();
    assert_eq!(append("/nonos/wifi/saved", b"x").map_err(errno), Err(ENODEV));
    assert_eq!(remove("/nonos/wifi/saved").map_err(errno), Err(ENODEV));
}

#[test]
fn a_store_read_from_the_loader_s_copy_with_no_disk_to_write_answers_no_device() {
    install(&image(&refs(&three()), Layout::Installer));
    disk::copy_only();
    assert_eq!(append("/nonos/wifi/saved", b"x").map_err(errno), Err(ENODEV));
    assert_eq!(remove("/nonos/setup/answers").map_err(errno), Err(ENODEV));
    assert_eq!(disk::landed(), 0, "nothing reached a disk");
}

#[test]
fn on_a_damaged_store_they_answer_that_it_is_damaged() {
    let mut img = image(&refs(&three()), Layout::Installer);
    img[8] ^= 0x01;
    install(&img);
    assert_eq!(append("/nonos/wifi/saved", b"x").map_err(errno), Err(EUCLEAN));
    assert_eq!(remove("/nonos/setup/answers").map_err(errno), Err(EUCLEAN));
    assert_eq!(disk::landed(), 0, "a refusal wrote nothing");
}

#[test]
fn a_keep_past_the_budget_or_a_full_table_is_no_space_and_writes_nothing() {
    let files = three();
    let mut img = image(&refs(&files), Layout::Installer);
    set_len(&mut img, 1, MAX_TOTAL_BYTES - 300);
    install(&img);
    assert_eq!(append("/nonos/new", &[1u8; 64]), Err(BlkError::NoSpace));
    assert_eq!(disk::landed(), 0);
    let names: Vec<String> = (0..MAX_ENTRIES).map(|i| format!("/f/{i}")).collect();
    let full: Vec<(&str, &[u8])> = names.iter().map(|n| (n.as_str(), &b"z"[..])).collect();
    install(&image(&full, Layout::Installer));
    assert_eq!(append("/nonos/new", b"z").map_err(errno), Err(ENOSPC));
    assert_eq!(disk::landed(), 0);
}

#[test]
fn a_keep_under_a_name_the_table_cannot_hold_is_a_bad_request() {
    install(&image(&refs(&three()), Layout::Installer));
    assert_eq!(append("/a\nb", b"x").map_err(errno), Err(EINVAL));
    assert_eq!(append("/a/../b", b"x").map_err(errno), Err(EINVAL));
    assert_eq!(disk::landed(), 0);
}
