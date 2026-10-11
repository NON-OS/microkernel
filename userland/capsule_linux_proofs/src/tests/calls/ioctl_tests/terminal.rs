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

//! Only a console on a terminal answers the terminal's requests.

use super::requests::{console, KINDS, NEVER, TTY};
use crate::linux::call::ioctl_req::{request, Req};
use crate::linux::guest::Kind;

#[test]
fn only_a_console_on_a_terminal_answers_the_terminal_requests() {
    for kind in KINDS {
        for on in [false, true] {
            for tty in TTY {
                let got = request(kind, on, tty);
                assert_eq!(got == Req::NotTty, !(on && console(kind)), "{tty:#x} on {on}");
            }
            for never in NEVER {
                assert_eq!(request(kind, on, never), Req::NotTty, "{never:#x}");
            }
        }
    }
    assert_eq!(request(Kind::Stdout, true, 0x5404), Req::SetTermios { flush: true });
    assert_eq!(request(Kind::Stdin, true, 0x5414), Req::SetWinsize);
    // ash sets the terminal's foreground group to its own at startup.
    assert_eq!(request(Kind::Stdin, true, 0x5410), Req::SetPgrp);
    // The request is an unsigned int: the upper half is not part of it.
    assert_eq!(request(Kind::Stdin, true, (7 << 32) | 0x5401), Req::GetTermios);
}
