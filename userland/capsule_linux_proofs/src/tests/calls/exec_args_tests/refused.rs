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

//! E2BIG for a vector too large, EFAULT for one that cannot be read.

use super::whole::{argv, BASE, E2BIG};
use crate::linux::abi::errno::EFAULT;
use crate::linux::call::exec_args::vector;

#[test]
fn one_argument_past_thirty_two_pages_is_e2big() {
    let at_most = vec![b'x'; 32 * 4096];
    assert_eq!(vector(&argv(1, &at_most, false), BASE).map(|v| v.len()), Ok(1));
    let over = vec![b'x'; 32 * 4096 + 1];
    assert_eq!(vector(&argv(1, &over, false), BASE), Err(E2BIG));
}

/// The whole vector is held to a quarter of the guest's stack.
#[test]
fn a_vector_past_its_total_is_e2big() {
    let arg = vec![b'x'; 4095];
    assert_eq!(vector(&argv(64, &arg, false), BASE).map(|v| v.len()), Ok(64));
    assert_eq!(vector(&argv(65, &arg, false), BASE), Err(E2BIG));
}

#[test]
fn a_vector_that_cannot_be_read_is_efault() {
    assert_eq!(vector(&argv(2, b"ls", false), BASE - 8), Err(EFAULT), "the table");
    assert_eq!(vector(&argv(2, b"ls", true), BASE), Err(EFAULT), "a pointer to nothing");
    let mem = argv(2, b"ls", false);
    mem.put(BASE, &(BASE - 64).to_le_bytes());
    assert_eq!(vector(&mem, BASE), Err(EFAULT), "a string outside the memory");
}
