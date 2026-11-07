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

//! A pack lands only with a trailer the machine would load it under: one
//! minted and refused, as on a machine setup left at "Only NONOS software",
//! stops the install with setup's reason, and one never minted keeps its
//! own.

use alloc::vec::Vec;
use core::cell::Cell;

use crate::install::admit::{admitted, Unadmitted};

const INDEX: &[u8] = b"a pack's header and index";

fn minted(head: &[u8]) -> Option<Vec<u8>> {
    Some([b"minted over ".as_slice(), head].concat())
}

#[test]
fn a_trailer_the_machine_serves_under_is_kept() {
    let asked = Cell::new(None);
    let serves = |head: &[u8], trailer: &[u8]| {
        asked.set(Some((head.to_vec(), trailer.to_vec())));
        true
    };
    assert_eq!(admitted(INDEX, minted, serves), Ok(minted(INDEX).unwrap()));
    let (head, trailer) = asked.take().expect("the kernel is asked");
    assert_eq!((head.as_slice(), trailer), (INDEX, minted(INDEX).unwrap()), "over the index minted");
}

#[test]
fn a_trailer_minted_and_refused_stops_the_install_with_setups_reason() {
    assert_eq!(admitted(INDEX, minted, |_, _| false), Err(Unadmitted::NotRun));
    let why = Unadmitted::NotRun.why();
    assert!(why.contains("Also software installed here"), "names setup's choice: {why}");
    assert!(why.contains("stops"), "says nothing more is fetched: {why}");
}

#[test]
fn no_trailer_is_not_asked_of_the_kernel() {
    let asked = Cell::new(false);
    let serves = |_: &[u8], _: &[u8]| {
        asked.set(true);
        true
    };
    assert_eq!(admitted(INDEX, |_| None, serves), Err(Unadmitted::NoMint));
    assert!(!asked.get());
    assert!(Unadmitted::NoMint.why().contains("could not be vouched for"));
}
