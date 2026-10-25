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

use std::cell::Cell;

use super::choice::{choose, Space};

const PROCESS_ASID: u32 = 4;
const SHARED_CR3: u64 = 0x7_000;
const SHARED_ASID: u32 = 9;

fn by_cr3(cr3: u64) -> Option<u32> {
    (cr3 == SHARED_CR3).then_some(SHARED_ASID)
}

#[test]
fn a_process_is_found_by_its_pid() {
    let read = Cell::new(false);
    let cr3 = || {
        read.set(true);
        Some(SHARED_CR3)
    };
    assert_eq!(choose(Some(PROCESS_ASID), cr3, by_cr3), Space::Asid(PROCESS_ASID));
    assert!(!read.get(), "its pcb's cr3 is not read");
}

#[test]
fn a_thread_is_found_by_its_process_tables() {
    // A thread owns no address-space entry; its pcb names its process's cr3.
    assert_eq!(choose(None, || Some(SHARED_CR3), by_cr3), Space::Asid(SHARED_ASID));
}

#[test]
fn an_heir_is_found_by_the_tables_it_inherited() {
    // The process that made the tables is gone; the heir's pcb still names them.
    let inherited = 0x9_000;
    let tables = |cr3: u64| (cr3 == inherited).then_some(12);
    assert_eq!(choose(None, || Some(inherited), tables), Space::Asid(12));
}

#[test]
fn neither_found_is_the_active_table_fallback() {
    assert_eq!(choose(None, || None, by_cr3), Space::ActiveTable, "no pcb");
    assert_eq!(choose(None, || Some(0), by_cr3), Space::ActiveTable, "a zero cr3");
    assert_eq!(choose(None, || Some(0x5_000), by_cr3), Space::ActiveTable, "unknown tables");
    let asked = Cell::new(false);
    let never = |_| {
        asked.set(true);
        Some(1)
    };
    assert_eq!(choose(None, || Some(0), never), Space::ActiveTable);
    assert!(!asked.get(), "a zero cr3 is not looked up");
}
