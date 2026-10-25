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

//! Clearing a record on a boot that keeps nothing.
//!
//! vfs lets zeros through its amnesic gate so that a record kept on an
//! earlier boot can still be withdrawn. Through the appender, zeros (or an
//! empty file) under a name the disk did not hold added a table row on an
//! amnesic boot. `withdraw` is the only path such a boot has to the disk: it
//! replaces a record already there, at its own length, and nothing else.

use nonos_disk_map::STORE_BASE_LBA;
use nonos_libc::disk;

use super::blk::error::BlkError;
use super::blk::store_withdraw::withdraw;
use super::fixture::{bytes, image, install, refs, three, Layout};

/// Every sector of the store the fixtures can reach.
const SPAN: usize = 4096;

fn untouched(before: &[u8]) {
    assert_eq!(disk::landed(), 0, "no sector was written");
    assert_eq!(disk::get(STORE_BASE_LBA, SPAN), before, "the store is as it was");
}

#[test]
fn zeros_under_a_name_the_disk_does_not_hold_write_nothing() {
    for layout in [Layout::Installer, Layout::Packer] {
        install(&image(&refs(&three()), layout));
        let before = disk::get(STORE_BASE_LBA, SPAN);
        for len in [1usize, 32, 512, 4097] {
            assert_eq!(
                withdraw("/nonos/consent/token", &vec![0u8; len]),
                Ok(()),
                "{layout:?} {len}"
            );
        }
        untouched(&before);
    }
}

#[test]
fn an_empty_file_under_a_new_name_writes_nothing() {
    install(&image(&refs(&three()), Layout::Packer));
    let before = disk::get(STORE_BASE_LBA, SPAN);
    assert_eq!(withdraw("/covert/channel/name-of-a-secret", &[]), Ok(()));
    untouched(&before);
}

#[test]
fn bytes_that_are_not_all_zero_are_refused_and_write_nothing() {
    let mut files = three();
    files[0] = (String::from("/nonos/wallet/vault"), bytes(1, 3000));
    install(&image(&refs(&files), Layout::Packer));
    let before = disk::get(STORE_BASE_LBA, SPAN);
    let mut almost = vec![0u8; 3000];
    almost[2999] = 1;
    assert_eq!(withdraw("/nonos/wallet/vault", &almost), Err(BlkError::Inval));
    assert_eq!(withdraw("/nonos/new", &bytes(9, 10)), Err(BlkError::Inval));
    untouched(&before);
}
