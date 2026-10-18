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

//! Which senders end a bound: any of the first four other senders it served,
//! by its next read (nvm::served). A sender that reads once and is gone ends
//! nothing, and the stream behind it still does; with the first sender alone
//! named, the one-off kept the bound and the stream was refused from 33 s
//! on, on a healthy drive. A fifth sender is not named: the bound lasts, and
//! its stream is refused once it passes, nothing sent.

use super::after::{after_a_recovery, streamed, Failed, ONE_OFF};
use super::inbox::Inbox;
use crate::protocol::E_TIMEDOUT;

const INSTALLER: u32 = 42;
const OTHER: u32 = 43;

/// After the kernel's recovery, a read from each of `senders`, queued as the
/// reply before it went; the first failure in the 90 s that follow.
fn after_reads_from(senders: &[u32]) -> Option<Failed> {
    let (mut s, first) = after_a_recovery(0);
    let mut inbox: Inbox = senders.iter().map(|&pid| (first.at, pid)).collect();
    streamed(&mut s, &mut inbox, first.at, 90_000)
}

#[test]
fn a_one_off_read_then_a_stream_is_served() {
    assert_eq!(after_reads_from(&[ONE_OFF, INSTALLER]), None, "refused when healthy");
}

#[test]
fn a_one_off_read_then_two_senders_in_turn_are_served() {
    assert_eq!(after_reads_from(&[ONE_OFF, INSTALLER, OTHER]), None, "refused when healthy");
}

#[test]
fn a_fifth_sender_is_not_named_and_its_stream_fails_safe() {
    // Four one-offs fill the set; the installer's stream behind them ends
    // nothing, and is refused, nothing sent, once the bound passes.
    let failed = after_reads_from(&[ONE_OFF, ONE_OFF + 1, ONE_OFF + 2, ONE_OFF + 3, INSTALLER]);
    let refused = matches!(failed, Some((ms, Err(E_TIMEDOUT), false)) if ms >= 33_000);
    assert!(refused, "{failed:?}");
}
