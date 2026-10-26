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

//! A signed schema 4 entry names its logo, and the market serves the bytes
//! it carries only when they are those.

use super::answer::{served, Served};
use super::table::embedded;
use crate::abi_v3::blobs::{blob, decode};
use crate::abi_v3::fixtures::sold;

fn signed_for(bytes: &[u8]) -> [u8; 32] {
    let mut index = sold();
    index.entries[0].logo_hash = *blake3::hash(bytes).as_bytes();
    decode(&blob(index)).expect("decodes").entries[0].logo_hash
}

#[test]
fn a_logo_is_served_when_its_bytes_are_the_signed_ones() {
    let git = embedded("linux.git").expect("the image carries git's logo");
    assert_eq!(served(&signed_for(git), Some(git)), Served::Logo(git));
}

#[test]
fn a_logo_whose_blake3_is_not_the_signed_one_is_refused() {
    let git = embedded("linux.git").expect("carried");
    let mut tampered = git.to_vec();
    tampered[0] ^= 1;
    assert_eq!(served(&signed_for(git), Some(&tampered)), Served::Refused);
    let vim = embedded("linux.vim").expect("carried");
    assert_eq!(served(&signed_for(git), Some(vim)), Served::Refused, "another listing's");
}

#[test]
fn a_raster_the_wire_cannot_carry_is_refused_even_when_signed() {
    let odd = [7u8; 4 * 49 * 49];
    assert_eq!(served(&signed_for(&odd), Some(&odd)), Served::Refused, "past the side");
    assert_eq!(served(&signed_for(&odd[..10]), Some(&odd[..10])), Served::Refused, "not square");
}

#[test]
fn no_signed_logo_or_no_carried_bytes_serves_none() {
    let git = embedded("linux.git").expect("carried");
    assert_eq!(served(&[0; 32], Some(git)), Served::None, "the entry names none");
    assert_eq!(served(&signed_for(git), None), Served::None, "an index newer than the image");
    assert_eq!(embedded("linux.age"), None, "age keeps its NONOS icon");
}
