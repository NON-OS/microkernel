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

//! The boot load, end to end, and the one answer it leaves in the status
//! word. A disk that is not there and a store that is there but wrong are
//! different answers, and only the second is one the desktop calls
//! corrupted: a live boot with no NONOS disk must not warn.

use nonos_libc::disk;

use super::blk::error::BlkError;
use super::blk::load::Load;
use super::blk::status;
use super::fixture::{image, install, refs, three, Layout};
use super::run::{load, shell_corrupt_codes, status_turn};

/// What the seeder leaves in the status word after a first attempt fails
/// with `e`.
fn code_after(e: BlkError) -> u32 {
    status::clear();
    status::record(&e);
    status::current()
}

#[test]
fn a_store_in_either_layout_loads_every_file() {
    let files = three();
    for layout in [Layout::Installer, Layout::Packer] {
        install(&image(&refs(&files), layout));
        assert_eq!(load(), Ok(files.clone()), "{layout:?}");
    }
}

#[test]
fn no_disk_is_no_service_and_not_corruption() {
    let _turn = status_turn();
    disk::reset();
    disk::remove();
    let e = Load::begin().err();
    assert_eq!(e, Some(BlkError::NoService));
    let code = code_after(BlkError::NoService);
    assert_eq!(code, 1);
    assert!(!shell_corrupt_codes().contains(&code), "a live boot would warn of corruption");
}

#[test]
fn a_device_that_times_out_or_refuses_is_not_corruption() {
    let _turn = status_turn();
    for (errno, want) in [(-110, BlkError::Transport(-110)), (-13, BlkError::Status(-13))] {
        install(&image(&refs(&three()), Layout::Installer));
        disk::fail_reads(errno);
        assert_eq!(Load::begin().err(), Some(want));
        let code = code_after(want);
        assert!(code != 0 && !shell_corrupt_codes().contains(&code), "{errno} reads as corrupt");
    }
}

#[test]
fn a_header_that_is_not_a_store_is_corruption() {
    let _turn = status_turn();
    let mut img = image(&refs(&three()), Layout::Installer);
    img[0] ^= 0x01;
    install(&img);
    assert_eq!(Load::begin().err(), Some(BlkError::BadContainer));
    assert!(shell_corrupt_codes().contains(&code_after(BlkError::BadContainer)));
}

#[test]
fn every_decode_failure_is_on_the_corrupt_side_of_the_shell_line() {
    let _turn = status_turn();
    let corrupt = shell_corrupt_codes();
    for e in [BlkError::ShortReply(1), BlkError::BadLength, BlkError::BadContainer] {
        assert!(corrupt.contains(&code_after(e)), "{e:?}");
    }
    for e in [BlkError::NoService, BlkError::Transport(-110), BlkError::Status(-5)] {
        assert!(!corrupt.contains(&code_after(e)), "{e:?}");
    }
}

#[test]
fn a_later_whole_load_clears_an_earlier_transient_failure() {
    let _turn = status_turn();
    status::clear();
    status::record(&BlkError::Transport(-110));
    status::record(&BlkError::BadContainer);
    assert_eq!(status::current(), 2, "the first failure is the evidence kept");
    status::clear();
    assert_eq!(status::current(), 0);
}
