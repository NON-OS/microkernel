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

//! An erase whose descriptor crosses a sector of the table.

use super::blk::store_withdraw::withdraw;
use super::fixture::{bytes, image, install, refs, three, without, Layout};
use super::run::{digests, load_counting};

/* Slot 3's descriptor runs from byte 416 to 544, across the table's first
 * sector boundary: both halves must go. */
#[test]
fn a_descriptor_across_two_sectors_is_erased_in_both() {
    let mut files = three();
    files.push((String::from("/nonos/wifi/saved"), bytes(9, 700)));
    files.push((String::from("/nonos/setup/done"), bytes(5, 8)));
    install(&image(&refs(&files), Layout::Packer));
    assert_eq!(withdraw("/nonos/wifi/saved", &[0u8; 700]), Ok(()));
    let got = load_counting().map(|(f, refused)| (digests(&f), refused));
    assert_eq!(got, Ok((digests(&without(&files, 3)), 0)));
}
