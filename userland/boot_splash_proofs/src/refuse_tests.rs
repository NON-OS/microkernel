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

//! Nothing at all is drawn when the logo was not on screen, when the record
//! and the file disagree in any field, or when the splash cannot hold it.

use crate::build::{bmp, record};
use crate::logo::prepare::{prepare, Why, MAX_FILE};
use crate::logo::record::{ON_SCREEN, TOP_DOWN};

#[test]
fn nothing_is_drawn_unless_the_record_and_file_agree() {
    let f = bmp(6, 4, 24, false);
    let go = |r: [u8; 28], f: &[u8]| prepare(&r, f, Some((20, 20)), (20, 20)).err();
    assert_eq!(go(record(0, 0, &f, 0, 0), &f), Some(Why::NotOnScreen));
    assert_eq!(go(record(0, 0, &f, ON_SCREEN | TOP_DOWN, 0), &f), Some(Why::Disagree));
    let mut r = record(0, 0, &f, ON_SCREEN, 0);
    r[18] = 32;
    assert_eq!(go(r, &f), Some(Why::Disagree), "bits per pixel");
    let mut longer = f.clone();
    longer.push(0);
    assert_eq!(go(record(0, 0, &f, ON_SCREEN, 0), &longer), Some(Why::Disagree), "file size");
    let mut r = record(0, 0, &f, ON_SCREEN, 0);
    r[8] = 7;
    assert_eq!(go(r, &f), Some(Why::Disagree), "width");
    assert_eq!(go(record(15, 0, &f, ON_SCREEN, 0), &f), Some(Why::Placement));
    let mut r = record(0, 0, &f, ON_SCREEN, 0);
    r[17] = 4;
    assert_eq!(go(r, &f), Some(Why::Record), "a fourth turn");
    assert_eq!(go(record(0, 0, &f, ON_SCREEN, 0), &f[..20]), Some(Why::File));
    assert_eq!(go(record(0, 0, &f, ON_SCREEN, 0), &vec![0; MAX_FILE + 1]), Some(Why::TooLarge));
}
