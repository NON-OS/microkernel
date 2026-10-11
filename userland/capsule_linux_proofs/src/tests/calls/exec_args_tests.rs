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

//! execve's argv and envp, read out of the guest. A vector is passed whole
//! or refused: E2BIG for one too large, as Linux answers, and EFAULT for
//! one it cannot read. A vector past the entry ceiling used to be cut
//! there, starting the program on arguments it was never given.

use super::fake_memory::Fake;
use crate::linux::abi::errno::EFAULT;
use crate::linux::call::exec_args::vector;
use crate::linux::guest::PAGE;

const BASE: u64 = 0x40_0000;
const E2BIG: i64 = 7;

/// A vector of `n` copies of `arg` at BASE, its strings after it, and the
/// terminating null pointer unless `open` leaves it off, in whole pages as
/// a guest holds memory.
fn argv(n: usize, arg: &[u8], open: bool) -> Fake {
    let table = (n + 1) * 8;
    let strings = BASE + table as u64;
    let mem = Fake::new(BASE, (table + arg.len() + 1).next_multiple_of(PAGE as usize));
    mem.put(strings, arg);
    mem.put(strings + arg.len() as u64, &[0]);
    for i in 0..n {
        mem.put(BASE + 8 * i as u64, &strings.to_le_bytes());
    }
    if open {
        mem.put(BASE + 8 * n as u64, &1u64.to_le_bytes());
    }
    mem
}

#[test]
fn a_vector_is_read_whole() {
    let mem = argv(3, b"ls", false);
    assert_eq!(vector(&mem, BASE), Ok(vec![b"ls".to_vec(); 3]));
    assert_eq!(vector(&mem, 0), Ok(Vec::new()), "a null vector is an empty one");
}

/// 4096 arguments are taken; the 4097th is E2BIG, never a vector cut short.
#[test]
fn a_vector_past_the_entry_ceiling_is_e2big_and_never_cut() {
    assert_eq!(vector(&argv(4096, b"a", false), BASE).map(|v| v.len()), Ok(4096));
    assert_eq!(vector(&argv(4097, b"a", false), BASE), Err(E2BIG));
}

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
