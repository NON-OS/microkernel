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

//! A record kept on an earlier boot, withdrawn: cleared at its own length,
//! refused at another, and the store loading whole at every cut.

use nonos_disk_map::STORE_BASE_LBA;
use nonos_libc::disk;

use super::blk::store_withdraw::withdraw;
use super::fixture::{bytes, image, install, refs, three, Layout};
use super::run::{digests, every_cut, load_counting};

/// Every sector of the store the fixtures can reach.
const SPAN: usize = 4096;

#[test]
fn a_kept_record_is_cleared_in_place_at_its_own_length() {
    for layout in [Layout::Installer, Layout::Packer] {
        let mut files = three();
        files[1] = (String::from("/nonos/consent/token"), bytes(4, 32));
        install(&image(&refs(&files), layout));
        assert_eq!(withdraw("/nonos/consent/token", &[0u8; 32]), Ok(()), "{layout:?}");
        let mut cleared = files.clone();
        cleared[1].1 = vec![0u8; 32];
        let got = load_counting().map(|(f, refused)| (digests(&f), refused));
        assert_eq!(
            got,
            Ok((digests(&cleared), 0)),
            "{layout:?}: the same names, one record zeroed"
        );
    }
}

#[test]
fn a_kept_record_of_another_length_is_refused_and_written_over_by_nothing() {
    let mut files = three();
    files[1] = (String::from("/nonos/consent/token"), bytes(4, 32));
    install(&image(&refs(&files), Layout::Packer));
    let before = disk::get(STORE_BASE_LBA, SPAN);
    assert!(withdraw("/nonos/consent/token", &[0u8; 64]).is_err());
    assert_eq!(disk::landed(), 0, "no sector was written");
    assert_eq!(disk::get(STORE_BASE_LBA, SPAN), before, "the store is as it was");
}

#[test]
fn a_withdrawal_cut_at_any_sector_leaves_the_old_record_or_the_zeros() {
    let mut files = three();
    files[0] = (String::from("/nonos/wallet/vault"), bytes(1, 3000));
    install(&image(&refs(&files), Layout::Packer));
    let mut cleared = files.clone();
    cleared[0].1 = vec![0u8; 3000];
    let (old, new) = (digests(&files), digests(&cleared));
    every_cut(&|| _ = withdraw("/nonos/wallet/vault", &[0u8; 3000]), &|cut, got| {
        let (got, refused) = got.expect("the store still loads");
        assert_eq!(refused, 0, "cut at {cut}");
        assert!(got == old || got == new, "cut at {cut}: {got:?}");
    });
}
