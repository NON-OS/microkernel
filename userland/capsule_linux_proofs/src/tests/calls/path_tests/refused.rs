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

//! The refusals, EFAULT and ENOENT, and which one a name meets first.

use super::super::fake_memory::Fake;
use super::read::{with, BASE};
use crate::linux::abi::errno::{EFAULT, ENAMETOOLONG, ENOENT};
use crate::linux::file::cstr::cstr;
use crate::linux::file::path::{name_of, path_of};
use crate::linux::guest::PAGE;

#[test]
fn a_pointer_the_guest_does_not_hold_is_efault() {
    let mem = with(BASE, b"/tmp/a");
    assert_eq!(path_of(&mem, 0), Err(EFAULT), "a null pointer");
    assert_eq!(path_of(&mem, BASE - 8), Err(EFAULT), "before the span");
    assert_eq!(path_of(&mem, u64::MAX - 2), Err(EFAULT), "where a page cannot end");
}

/// Memory that ends before the terminator is EFAULT while the name is
/// still inside the ceiling, and ENAMETOOLONG once it is past it, nothing
/// past the ceiling being read.
#[test]
fn which_comes_first_decides_the_errno() {
    let end = BASE + 2 * PAGE;
    let short = Fake::new(BASE, 2 * PAGE as usize);
    short.put(end - 10, &[b'x'; 10]);
    assert_eq!(path_of(&short, end - 10), Err(EFAULT), "runs off the end inside the ceiling");
    let long = Fake::new(BASE, 2 * PAGE as usize);
    long.put(end - 300, &[b'x'; 300]);
    assert_eq!(path_of(&long, end - 300), Err(ENAMETOOLONG), "past the ceiling first");
    assert_eq!(cstr(&long, end - 300, 400), Err(EFAULT), "with a higher ceiling, the fault");
}

#[test]
fn an_empty_name_is_enoent_where_a_call_acts_on_it() {
    let mem = with(BASE, b"");
    assert_eq!(path_of(&mem, BASE), Ok(Vec::new()), "stat's AT_EMPTY_PATH reads it");
    assert_eq!(name_of(&mem, BASE), Err(ENOENT));
}
