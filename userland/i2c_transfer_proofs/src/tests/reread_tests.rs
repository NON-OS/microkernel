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

//! The read-back of an LPSS function's own record after its claim, against
//! lists that do not show its window: the function is refused before any
//! register is written, and the remap never names address 0. And a list
//! whose order changed, or that gained a record before the function's, is
//! still read by the function's device_id, never by its place.

use nonos_libc::{list_devices, BAR_KIND_PIO};

use super::listed::{discovered, list, seen, table};
use super::records::{lpss, platform, WINDOW};
use super::remap_tests::{bring, core, remaps};
use crate::regs;

#[test]
fn an_lpss_function_whose_record_shows_no_window_once_claimed_is_refused_untouched() {
    let board = table(&[lpss(0), lpss(1)], &[]);
    list(&board, None);
    let (found, _) = discovered();
    let (mut pio, mut empty) = (board[1], board[1]);
    (pio.bars[0].kind, empty.bars[0].size) = (BAR_KIND_PIO, 0);
    let cases = [
        ("the list failed", Err(-14)),
        ("its record is gone", Ok(seen(&board[..1], Some(1)))),
        ("its base still reads 0", Ok(seen(&board, None))),
        ("its bar 0 is no MMIO window", Ok(seen(&[board[0], pio], Some(1)))),
        ("its bar 0 has no size", Ok(seen(&[board[0], empty], Some(1)))),
    ];
    for (why, listed) in cases {
        list_devices(listed);
        let _core = core();
        let refused = bring(&found[1]).expect_err(why);
        assert!(refused.contains("not remapped"), "{why}: {refused}");
        assert!(regs::with(|c| c.events().is_empty()), "{why}: a register was written");
    }
}

#[test]
fn a_record_moved_or_preceded_by_another_is_still_read_back_by_its_id() {
    let board = table(&[lpss(0), lpss(1)], &[]);
    list(&board, None);
    let (found, _) = discovered();
    let moved = [board[1], board[0]];
    let preceded = [platform(5), board[1], board[0]];
    for (why, after) in [("moved", &moved[..]), ("preceded", &preceded[..])] {
        list(after, Some(0));
        let _core = core();
        assert_eq!(bring(&found[0]), Ok(()), "{why}");
        assert_eq!(remaps(), [WINDOW], "{why}: another record's base");
    }
}
