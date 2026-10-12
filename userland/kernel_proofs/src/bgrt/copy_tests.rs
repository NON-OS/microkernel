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

//! The logo copy, through the aarch64 mount of the one file: kept when
//! the headers and the whole agree; refused, reading no further, when the
//! headers fail or the file is too large; and when memory cannot be read.

use super::bmp_tests::file;
use super::tests::table;
use crate::arm_acpi::bgrt::bgrt_copy::{copy_logo, LogoError, MAX_IMAGE};
use std::cell::RefCell;

const AT: u64 = 0x7E00_0000;

/// A reader over `image` at AT, logging each read's length.
fn run(t: &[u8], image: &[u8]) -> (Result<(u32, usize), LogoError>, Vec<usize>) {
    let log = RefCell::new(Vec::new());
    let read = |phys: u64, len: usize| {
        log.borrow_mut().push(len);
        (phys == AT).then(|| image.get(..len).map(<[u8]>::to_vec)).flatten()
    };
    let r = copy_logo(t, read).map(|(_, m, bytes)| (m.width, bytes.len()));
    (r, log.into_inner())
}

#[test]
fn kept_when_both_checks_agree() {
    let f = file(3, 2, 24);
    assert_eq!(run(&table(1, 1, AT), &f), (Ok((3, 78)), vec![54, 78]));
}

#[test]
fn refused_and_read_no_further() {
    let mut bad = file(3, 2, 24);
    bad[0] = b'X';
    assert_eq!(run(&table(1, 1, AT), &bad), (Err(LogoError::NotDrawable), vec![54]));
    let mut huge = file(3, 2, 24);
    huge[2..6].copy_from_slice(&(MAX_IMAGE + 1).to_le_bytes());
    assert_eq!(run(&table(1, 1, AT), &huge).0, Err(LogoError::TooLarge));
    assert_eq!(run(&table(1, 1, AT + 1), &file(3, 2, 24)).0, Err(LogoError::Unreadable));
    assert_eq!(run(&table(2, 1, AT), &file(3, 2, 24)), (Err(LogoError::NoLogo), vec![]));
    let short = file(3, 2, 24);
    assert_eq!(run(&table(1, 1, AT), &short[..70]).0, Err(LogoError::Unreadable), "file cut short");
}
