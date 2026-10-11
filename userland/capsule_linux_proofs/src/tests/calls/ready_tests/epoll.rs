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

//! epoll's level and edge, and EPOLLEXCLUSIVE.

use super::select::{HUP, IN, OUT};
use crate::linux::abi::errno::EINVAL;
use crate::linux::file::epoll_rules::{exclusive_ok, live, seen};
use crate::linux::guest::{EPOLLET, EPOLLONESHOT};

const EXCLUSIVE: u32 = 1 << 28;
const ADD: u64 = 1;
const DEL: u64 = 2;
const MOD: u64 = 3;

/// op, events, whether the target is an epoll, the entry there, and the answer.
type ExclusiveRow = (u64, u32, bool, Option<u32>, Result<(), i64>);

#[test]
fn an_epoll_entry_reports_level_and_edge_as_linux_does() {
    assert_eq!(live(IN | OUT, IN, 0), IN, "only what was asked for");
    assert_eq!(live(HUP, IN, 0), HUP, "hang-up is reported unasked");
    assert_eq!(live(IN, IN, IN), IN, "level-triggered: for as long as it holds");
    assert_eq!(live(IN, IN | EPOLLET, IN), 0, "edge-triggered: once per rise");
    assert_eq!(live(IN | OUT, IN | OUT | EPOLLET, IN), OUT, "a new edge is reported");
    assert_eq!(seen(IN | OUT | HUP, OUT), OUT | HUP);
}

#[test]
fn epollexclusive_is_allowed_only_where_linux_allows_it() {
    let table: [ExclusiveRow; 8] = [
        (ADD, EXCLUSIVE | IN | EPOLLET, false, None, Ok(())),
        (ADD, EXCLUSIVE | IN | EPOLLONESHOT, false, None, Err(EINVAL)),
        (ADD, EXCLUSIVE | IN, true, None, Err(EINVAL)),
        (MOD, EXCLUSIVE | IN, false, Some(IN), Err(EINVAL)),
        (MOD, IN, false, Some(EXCLUSIVE | IN), Err(EINVAL)),
        (MOD, IN | OUT, false, Some(IN), Ok(())),
        (ADD, IN | EPOLLONESHOT, true, None, Ok(())),
        (DEL, EXCLUSIVE, false, Some(EXCLUSIVE), Ok(())),
    ];
    for (op, events, target_epoll, present, want) in table {
        assert_eq!(exclusive_ok(op, events, target_epoll, present), want, "op {op} {events:#x}");
    }
}
