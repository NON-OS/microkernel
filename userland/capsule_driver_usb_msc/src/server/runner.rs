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

use nonos_libc::Deadline;

use crate::scan::{Medium, Scanner, RESCAN_MS};
use crate::server::inbox::Inbox;
use crate::state::State;

/// While the device is being looked for, requests are answered between
/// passes: `E_AGAIN` for the block surface, as usual for the rest.
const SCAN_POLL_MS: u64 = 5;
/// With no device after the window, how long a receive waits before the
/// ports are looked at again.
const ABSENT_POLL_MS: u64 = 250;

pub fn run() -> ! {
    let mut inbox = Inbox::new();
    let mut state = State::new();
    let mut scanner = Scanner::new();
    let mut medium = Medium::Scanning;
    let mut rescan = Deadline::after_ms(RESCAN_MS);
    loop {
        let wait = match medium {
            Medium::Scanning => SCAN_POLL_MS,
            Medium::Absent => ABSENT_POLL_MS,
            Medium::Ready(_) => 0,
        };
        inbox.serve(wait, &mut state, (&medium, &scanner.report), false);
        // A probe answers the inbox in its pauses (`Inbox::pause`).
        let mut pause = |s: &mut State, r: &_| inbox.pause(s, r);
        if matches!(medium, Medium::Scanning) {
            medium = scanner.step(&mut state, &mut pause);
        } else if matches!(medium, Medium::Absent) && rescan.expired() {
            medium = scanner.again(&mut state, &mut pause);
            rescan = Deadline::after_ms(RESCAN_MS);
        }
    }
}
