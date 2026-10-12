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

//! The extension block, read from the end: the project instruction is
//! found past others, and a block that ends, or holds a zero length, before
//! naming one names none.

use super::fixture::{epatch, VERSION};
use super::rtl::project::project;

fn body() -> Vec<u8> {
    let mut f = epatch(VERSION, &[(1, &[0; 8])], 0);
    f.truncate(f.len() - 8);
    f
}

fn with_block(block: &[u8]) -> Vec<u8> {
    let mut f = body();
    f.extend_from_slice(block);
    f.extend_from_slice(&[0x51, 0x04, 0xfd, 0x77]);
    f
}

#[test]
fn the_project_is_the_one_octet_of_opcode_zero() {
    assert_eq!(project(&epatch(VERSION, &[(1, &[0; 8])], 13)), Some(13));
    assert_eq!(project(&with_block(&[0xff, 0x0a, 0x01, 0x00])), Some(0x0a));
}

#[test]
fn other_instructions_are_stepped_over() {
    // From the end: opcode 2 with three octets, then the project.
    let block = [0xff, 0x0e, 0x01, 0x00, 0xaa, 0xbb, 0xcc, 0x03, 0x02];
    assert_eq!(project(&with_block(&block)), Some(0x0e));
    // One octet under opcode 5 is not a project.
    assert_eq!(project(&with_block(&[0x0e, 0x01, 0x00, 0x33, 0x01, 0x05])), Some(0x0e));
}

#[test]
fn no_signature_no_project() {
    let mut f = epatch(VERSION, &[(1, &[0; 8])], 13);
    let last = f.len() - 1;
    f[last] = 0x78;
    assert_eq!(project(&f), None);
    assert_eq!(project(&[0x51, 0x04, 0xfd]), None);
}

#[test]
fn an_end_or_a_zero_length_first_names_no_project() {
    assert_eq!(project(&with_block(&[0x0a, 0x01, 0x00, 0x00, 0xff])), None, "ends");
    let past_end = [0x0a, 0x01, 0x00, 0x11, 0x22, 0x02, 0xff];
    assert_eq!(project(&with_block(&past_end)), None, "nothing is read past the end");
    assert_eq!(project(&with_block(&[0x0a, 0x01, 0x00, 0x00, 0x03])), None, "zero length");
    assert_eq!(project(&with_block(&[0x09, 0x02, 0x7f, 0x05])), None, "runs off the block");
}
