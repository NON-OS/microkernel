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

//! Running actions: each instruction kind against a recording bus, writes
//! refused untouched with the switch off, preserve, and the errors.

use super::build::{entry, table, Bus, IO};
use super::wdat_action::*;
use super::wdat_entry::wdat;
use super::wdat_run::{run, RunError};

fn bus(regs: &[(u64, u64)]) -> Bus {
    Bus { regs: regs.iter().copied().collect(), log: Vec::new() }
}

#[test]
fn reads_answer_without_the_switch() {
    let e = [
        entry(QUERY_RUNNING, 0, IO, 0x10, 1, 1),
        entry(QUERY_CURRENT_COUNTDOWN, 1, IO, 0x14, 0, 0xFFFF),
    ];
    let w = wdat(&table(&e, 2)).unwrap();
    let mut b = bus(&[(0x10, 0xF1), (0x14, 0xAB_1234)]);
    assert_eq!(run(&w.entries, QUERY_RUNNING, 0, false, &mut b), Ok(1));
    assert_eq!(run(&w.entries, QUERY_CURRENT_COUNTDOWN, 0, false, &mut b), Ok(0x1234));
    b.regs.insert(0x10, 0xF0);
    assert_eq!(run(&w.entries, QUERY_RUNNING, 0, false, &mut b), Ok(0));
    assert!(b.log.iter().all(|(k, _, _)| *k == 'r'));
}

#[test]
fn writes_wait_for_the_switch() {
    let e = [
        entry(RESET, 2, IO, 0x20, 1, 1),
        entry(SET_COUNTDOWN, 3 | 0x80, IO, 0x24, 0, 0xFFFF),
        entry(SET_COUNTDOWN, 0, IO, 0x28, 1, 1),
    ];
    let w = wdat(&table(&e, 3)).unwrap();
    let mut b = bus(&[(0x20, 0), (0x24, 0xAB00_0000), (0x28, 1)]);
    assert_eq!(run(&w.entries, RESET, 0, false, &mut b), Err(RunError::Switched));
    assert_eq!(run(&w.entries, SET_COUNTDOWN, 300, false, &mut b), Err(RunError::Switched));
    assert!(b.log.is_empty(), "refused before any register is touched");
    assert_eq!(run(&w.entries, RESET, 0, true, &mut b), Ok(0));
    assert_eq!(b.log, [('w', 0x20, 1)]);
    b.log.clear();
    assert_eq!(run(&w.entries, SET_COUNTDOWN, 300, true, &mut b), Ok(1));
    assert_eq!(b.log, [('r', 0x24, 0xAB00_0000), ('w', 0x24, 0xAB00_012C), ('r', 0x28, 1)]);
}

#[test]
fn no_action_and_an_unreachable_register() {
    let w = wdat(&table(&[entry(QUERY_STATUS, 0, IO, 0x30, 1, 1)], 1)).unwrap();
    let mut b = bus(&[]);
    assert_eq!(run(&w.entries, SET_STATUS, 0, true, &mut b), Err(RunError::NoAction));
    assert_eq!(run(&w.entries, QUERY_STATUS, 0, false, &mut b), Err(RunError::Bus));
}
