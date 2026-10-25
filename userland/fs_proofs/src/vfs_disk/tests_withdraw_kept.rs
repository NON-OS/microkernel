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

//! A record kept on an earlier boot, withdrawn on a boot that keeps nothing:
//! erased where it lies, refused at another length, the store loading whole
//! at every cut, and no byte written but a zero over what was there.

use nonos_disk_map::STORE_BASE_LBA;
use nonos_libc::disk;

use super::blk::store_withdraw::withdraw;
use super::fixture::{bytes, image, install, refs, three, without, Layout};
use super::run::{digests, every_cut, load_counting};

/// Every sector of the store the fixtures can reach.
const SPAN: usize = 4096;

/// What the kernel lets a boot that keeps nothing write: each change a zero.
fn erased_only(before: &[u8], after: &[u8]) -> bool {
    before.iter().zip(after).all(|(b, a)| a == b || *a == 0)
}

#[test]
fn a_kept_record_is_erased_where_it_lies() {
    for layout in [Layout::Installer, Layout::Packer] {
        let mut files = three();
        files[1] = (String::from("/nonos/consent/token"), bytes(4, 32));
        install(&image(&refs(&files), layout));
        let before = disk::get(STORE_BASE_LBA, SPAN);
        assert_eq!(withdraw("/nonos/consent/token", &[0u8; 32]), Ok(()), "{layout:?}");
        let after = disk::get(STORE_BASE_LBA, SPAN);
        assert!(erased_only(&before, &after), "{layout:?}: a byte became other than zero");
        let got = load_counting().map(|(f, refused)| (digests(&f), refused));
        let left = without(&files, 1);
        assert_eq!(got, Ok((digests(&left), 0)), "{layout:?}: gone, and nothing called damage");
        assert_eq!(withdraw("/nonos/consent/token", &[0u8; 32]), Ok(()), "a second time");
        assert_eq!(disk::get(STORE_BASE_LBA, SPAN), after, "a second time writes nothing");
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
fn a_withdrawal_cut_at_any_sector_leaves_the_record_or_nothing_of_it() {
    let mut files = three();
    files[0] = (String::from("/nonos/wallet/vault"), bytes(1, 3000));
    install(&image(&refs(&files), Layout::Packer));
    let (old, gone) = (digests(&files), digests(&without(&files, 0)));
    every_cut(&|| _ = withdraw("/nonos/wallet/vault", &[0u8; 3000]), &|cut, got| {
        let (got, _) = got.expect("the store still loads");
        assert!(got == old || got == gone, "cut at {cut}: {got:?}");
    });
}
