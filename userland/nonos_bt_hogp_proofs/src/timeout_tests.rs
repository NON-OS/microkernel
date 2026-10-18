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

//! The ATT transaction timeout (Vol 3, Part F, 3.3.3).

use crate::sim::{keyboard_mouse, Peripheral, Run};
use nonos_bt_hogp::host::{Outcome, Step, TIMEOUT_MS};
use nonos_bt_hogp::HogpError;

#[test]
fn thirty_silent_seconds_end_the_bearer() {
    let mut peer = Peripheral::new(keyboard_mouse());
    peer.silent = true;
    let mut run = Run::with(peer);
    assert_eq!(run.go(), None);
    assert_eq!(run.host.poll(TIMEOUT_MS - 1), Step::default());
    let timeout = Some(Outcome::Failed(HogpError::Timeout));
    assert_eq!(run.host.poll(TIMEOUT_MS).outcome, timeout);
    // Nothing is sent on the bearer after that, not even a refusal.
    assert_eq!(run.host.on_pdu(&[0x0a, 0x03, 0x00], TIMEOUT_MS + 1), Step::default());
}
