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

//! The watchdog header, and every action WDAT names decoded with its
//! kind, register, value and mask.

use super::build::{entry, table, IO};
use super::wdat_action::*;
use super::wdat_decode::{Kind, Reg};
use super::wdat_entry::wdat;

const ACTIONS: [u8; 14] = [
    RESET,
    QUERY_CURRENT_COUNTDOWN,
    QUERY_COUNTDOWN,
    SET_COUNTDOWN,
    QUERY_RUNNING,
    SET_RUNNING,
    QUERY_STOPPED,
    SET_STOPPED,
    QUERY_REBOOT,
    SET_REBOOT,
    QUERY_SHUTDOWN,
    SET_SHUTDOWN,
    QUERY_STATUS,
    SET_STATUS,
];

#[test]
fn the_watchdog_header() {
    let w = wdat(&table(&[], 0)).expect("header");
    assert_eq!(w.pci, (0xFFFF, 0xFF, 0xFF, 0xFF));
    assert_eq!((w.period_ms, w.max_count, w.min_count), (1000, 600, 1));
    assert!(w.enabled && w.stopped_in_sleep);
    assert!(w.entries.is_empty() && w.malformed == 0);
}

#[test]
fn every_action_and_kind_decoded() {
    let all: Vec<_> = ACTIONS
        .iter()
        .enumerate()
        .map(|(n, a)| entry(*a, (n as u8 % 4) | ((n as u8 & 4) << 5), IO, 0x400 + n as u64, 7, 0xF))
        .collect();
    let w = wdat(&table(&all, all.len() as u32)).expect("table");
    assert_eq!(w.entries.len(), ACTIONS.len());
    let kinds = [Kind::ReadValue, Kind::ReadCountdown, Kind::WriteValue, Kind::WriteCountdown];
    for (n, i) in w.entries.iter().enumerate() {
        assert_eq!(i.action, ACTIONS[n]);
        assert_eq!(i.kind, kinds[n % 4]);
        assert_eq!(i.preserve, n & 4 != 0);
        let reg = Reg {
            space: IO,
            bit_width: 32,
            bit_offset: 0,
            access_size: 3,
            address: 0x400 + n as u64,
        };
        assert_eq!((i.reg, i.value, i.mask), (reg, 7, 0xF));
    }
}
