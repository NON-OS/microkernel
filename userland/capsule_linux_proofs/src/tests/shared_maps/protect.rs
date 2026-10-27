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

//! mprotect on a shared file map: never exec, never write a descriptor was
//! not opened for, and a map given write is a writer for good, under the
//! one writer rule.

use super::support::{map, may, P};
use crate::linux::abi::errno::{EACCES, ENOSYS};
use crate::linux::file::shared_maps::asks::{protects, writers};

#[test]
fn mprotect_never_gives_exec_nor_write_a_descriptor_did_not_have() {
    let all = vec![map(1, 16, 4, "/f", 0), may(map(1, 32, 4, "/g", 0))];
    assert_eq!(protects(&all, 1, 17 * P, 18 * P, (false, true)), Err(EACCES));
    assert_eq!(protects(&all, 1, 17 * P, 18 * P, (true, false)), Err(EACCES));
    assert_eq!(protects(&all, 1, 33 * P, 34 * P, (true, false)), Ok(()));
    assert_eq!(protects(&all, 2, 17 * P, 18 * P, (true, true)), Ok(()), "a fork's copy");
    assert_eq!(protects(&all, 1, 20 * P, 32 * P, (true, true)), Ok(()), "between the maps");
    assert_eq!(protects(&all, 1, 16 * P, 36 * P, (false, false)), Ok(()), "PROT_READ");
}

#[test]
fn becoming_a_writer_holds_the_rule_and_sticks() {
    let mut all = vec![may(map(1, 16, 4, "/f", 0)), map(2, 16, 4, "/f", 2)];
    assert_eq!(protects(&all, 1, 16 * P, 17 * P, (true, false)), Err(ENOSYS));
    all.pop();
    writers(&mut all, 1, 19 * P, 20 * P);
    assert!(all[0].writer, "any page of it given write makes it a writer");
    writers(&mut all, 1, 0, 16 * P);
    assert!(all[0].writer);
}
