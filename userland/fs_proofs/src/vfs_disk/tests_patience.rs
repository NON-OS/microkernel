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


//! When the boot load stops trying (capsule_vfs blk/patience.rs). A stick
//! driver.usb_msc0 finds ten seconds after xHCI is up must still be read,
//! and a store that is there and wrong must not hold the answer back.

use nonos_libc::disk;

use super::blk::error::BlkError;
use super::blk::load::Load;
use super::blk::patience::{gives_up, not_yet, DISK_WAIT_MS, MAX_ATTEMPTS};
use super::fixture::{image, install, refs, three, Layout};
use super::run::{load, status_turn};

/// The USB driver's own search window (capsule_driver_usb_msc scan/scanner.rs).
const USB_SEARCH_MS: i64 = 10_000;

#[test]
fn a_disk_not_there_yet_is_waited_for_past_the_usb_search() {
    for e in [BlkError::NoService, BlkError::Transport(-110)] {
        assert!(not_yet(&e), "{e:?}");
        assert!(!gives_up(&e, MAX_ATTEMPTS, 5_000), "{e:?} given up on at five seconds");
        assert!(!gives_up(&e, 40, USB_SEARCH_MS + 2_000), "{e:?} given up on before the stick is found");
        assert!(gives_up(&e, MAX_ATTEMPTS, DISK_WAIT_MS), "{e:?} waited for forever");
    }
    const { assert!(DISK_WAIT_MS >= 2 * USB_SEARCH_MS) };
}

#[test]
fn a_store_that_is_there_and_wrong_is_given_up_on_after_a_few_attempts() {
    for e in [BlkError::BadContainer, BlkError::Inval, BlkError::BadLength, BlkError::Status(-13)] {
        assert!(!not_yet(&e), "{e:?}");
        assert!(!gives_up(&e, MAX_ATTEMPTS - 1, 0), "{e:?}");
        assert!(gives_up(&e, MAX_ATTEMPTS, 0), "{e:?} held the answer back");
    }
}

#[test]
fn the_first_attempts_are_never_the_last() {
    for e in [BlkError::NoService, BlkError::BadContainer] {
        assert!(!gives_up(&e, 1, DISK_WAIT_MS * 10), "{e:?}");
    }
}

/*
 * The boot a stick gives: no disk answers until driver.usb_msc0 is up, then
 * reads time out while it searches, as the kernel answers them, then the
 * store is there. No attempt before it is the last, and the one after loads
 * every file.
 */
#[test]
fn a_stick_found_late_is_loaded_whole() {
    let _turn = status_turn();
    let files = three();
    let mut attempts = 0;
    let mut waited = 0;
    for phase in 0..2 {
        disk::reset();
        match phase {
            0 => disk::remove(),
            _ => disk::fail_reads(-110),
        }
        for _ in 0..4 {
            attempts += 1;
            let e = Load::begin().err().expect("no store yet");
            assert!(!gives_up(&e, attempts, waited), "gave up at attempt {attempts}, {waited} ms");
            waited += 1_750;
        }
    }
    assert!(waited > USB_SEARCH_MS);
    install(&image(&refs(&files), Layout::Installer));
    assert_eq!(load(), Ok(files));
}
