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

//! An index may inflate past the inflater's 4 MiB default: Alpine's
//! community index is 8 MB and Kali's Packages 85 MB, and on a live boot the
//! default refused both after they had downloaded and verified. The
//! installer's bound admits them and still stops a stream that runs past it.

use crate::install::unpacked::{decompressed, MAX_INFLATED};

const LARGE: &[u8] = include_bytes!("../../vectors/inflate/large.gz");
const SIX_MIB: usize = 6 << 20;

#[test]
fn the_default_bound_is_what_refused_a_real_index() {
    assert_eq!(nonos_inflate::gunzip(LARGE), None);
    assert_eq!(nonos_inflate::members(LARGE).map(|m| m.len()), None);
}

#[test]
fn the_installer_bound_opens_it_whole() {
    assert_eq!(decompressed(LARGE).map(|b| b.len()), Some(SIX_MIB));
    let parts = nonos_inflate::members_within(LARGE, MAX_INFLATED).expect("two members");
    assert_eq!(parts.iter().map(|m| m.body.len()).sum::<usize>(), SIX_MIB);
    assert_eq!((parts.len(), parts[1].end), (2, LARGE.len()));
}

#[test]
fn a_bound_below_the_output_still_refuses() {
    assert_eq!(nonos_inflate::gunzip_within(LARGE, SIX_MIB - 1), None);
    assert_eq!(nonos_inflate::members_within(LARGE, SIX_MIB - 1).map(|m| m.len()), None);
    assert!(nonos_inflate::gunzip_within(LARGE, SIX_MIB).is_some());
}
