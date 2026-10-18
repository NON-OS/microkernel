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

//! Names read back whole, up to the ceiling and across pages.

use super::super::fake_memory::Fake;
use crate::linux::abi::errno::ENAMETOOLONG;
use crate::linux::file::path::{name_of, path_of, MAX_PATH};
use crate::linux::guest::PAGE;

pub(super) const BASE: u64 = 0x40_0000;

/// Two pages of guest memory with `name` and its terminator at `at`.
pub(super) fn with(at: u64, name: &[u8]) -> Fake {
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
