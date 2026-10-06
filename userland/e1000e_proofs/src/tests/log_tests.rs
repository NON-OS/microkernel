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

//! The exact console text the owner is asked to photograph.

use nonos_libc::{clear_log, logged};

use crate::constants::status::{STATUS_FD, STATUS_LU};
use crate::log::{say, Line};
use crate::server::link_log::log_change;

#[test]
fn link_changes_read_as_speed_and_duplex() {
    clear_log();
    log_change(0);
    log_change(STATUS_LU | STATUS_FD | (2 << 6));
    log_change(STATUS_LU | STATUS_FD | (3 << 6));
    log_change(STATUS_LU | (1 << 6));
    log_change(STATUS_LU | STATUS_FD);
    assert_eq!(
        logged(),
        [
            "e1000e: link down",
            "e1000e: link up 1000 full",
            "e1000e: link up 1000 full",
            "e1000e: link up 100 half",
            "e1000e: link up 10 full",
        ]
    );
}

#[test]
fn ids_and_addresses_print_as_lowercase_hex() {
    clear_log();
    Line::new().text("up ").hex16(0x0D4E).text(" mac ").mac(&[0x02, 0xAB, 0, 9, 0x10, 0xFF]).send();
    say("reset did not clear in 50 ms");
    assert_eq!(
        logged(),
        ["e1000e: up 0d4e mac 02:ab:00:09:10:ff", "e1000e: reset did not clear in 50 ms"]
    );
}

#[test]
fn an_overlong_line_is_cut_and_still_ends_its_line() {
    clear_log();
    let long = "x".repeat(400);
    say(&long);
    let line = &logged()[0];
    assert!(line.starts_with("e1000e: xxx"));
    assert_eq!(line.len(), 159, "160 bytes with the newline");
}
