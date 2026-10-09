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

//! A path read out of a guest answers what Linux's getname answers: the
//! name, EFAULT for a pointer it cannot follow, ENAMETOOLONG for a name
//! past the ceiling, and ENOENT for an empty name a call acts on, where
//! every refusal used to be EFAULT and an empty name was the working
//! directory.

use super::fake_memory::Fake;
use crate::linux::abi::errno::{EFAULT, ENAMETOOLONG, ENOENT};
use crate::linux::file::cstr::cstr;
use crate::linux::file::path::{name_of, path_of, MAX_PATH};
use crate::linux::guest::PAGE;

const BASE: u64 = 0x40_0000;

/// Two pages of guest memory with `name` and its terminator at `at`.
fn with(at: u64, name: &[u8]) -> Fake {
    let mem = Fake::new(BASE, 2 * PAGE as usize);
    mem.put(at, name);
    mem.put(at + name.len() as u64, &[0]);
    mem
}

#[test]
fn a_name_reads_back_whole() {
    let mem = with(BASE, b"/tmp/a");
    assert_eq!(path_of(&mem, BASE), Ok(b"/tmp/a".to_vec()));
    assert_eq!(name_of(&mem, BASE), Ok(b"/tmp/a".to_vec()));
}

#[test]
fn a_name_at_the_ceiling_is_taken_and_one_past_it_is_enametoolong() {
    let longest = vec![b'a'; MAX_PATH];
    assert_eq!(path_of(&with(BASE, &longest), BASE), Ok(longest.clone()));
    let over = vec![b'a'; MAX_PATH + 1];
    assert_eq!(path_of(&with(BASE, &over), BASE), Err(ENAMETOOLONG));
    assert_eq!(name_of(&with(BASE, &over), BASE), Err(ENAMETOOLONG));
}

/// A name that crosses a page boundary reads on into the next page.
#[test]
fn a_name_across_a_page_boundary_is_read_whole() {
    let at = BASE + PAGE - 3;
    assert_eq!(path_of(&with(at, b"/home/u"), at), Ok(b"/home/u".to_vec()));
}

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
