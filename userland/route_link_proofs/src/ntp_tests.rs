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

//! net.ntp at boot and after: a time server is asked only when Direct is
//! the default network, an unreadable store counts as the mixnet, and a
//! skip is said once per reason rather than every time it is looked at.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::direct_only::direct_refused;
use crate::ntp_decide::{skip_line, step, Step};

#[test]
fn a_time_server_is_asked_only_under_direct() {
    let defaults = core::iter::once(None).chain((0..=u8::MAX).map(Some));
    for default in defaults {
        for said in [None, direct_refused(Some(NYM)), direct_refused(None)] {
            let asks = step(direct_refused(default), said) == Step::Sync;
            assert_eq!(asks, default == Some(DIRECT), "{default:?}");
        }
    }
}

#[test]
fn before_the_store_is_up_nothing_is_asked() {
    /* Unreadable is the default, the Nym mixnet: skipped, and said. */
    match step(direct_refused(None), None) {
        Step::Skip { why, log } => {
            assert!(log);
            assert!(why.contains("could not be read") && why.contains("Nym mixnet"));
        }
        Step::Sync => panic!("a time server was asked before the store answered"),
    }
}

#[test]
fn a_skip_is_said_once_per_reason() {
    let nym = direct_refused(Some(NYM));
    let anyone = direct_refused(Some(ANYONE));
    /* Boot, store not up yet: said. Looked at again: not said again. */
    let mut said = None;
    let mut lines = 0;
    for refused in [direct_refused(None), direct_refused(None), nym, nym, nym, anyone, anyone] {
        if let Step::Skip { why, log } = step(refused, said) {
            lines += usize::from(log);
            said = Some(why);
        }
    }
    assert_eq!(lines, 3, "unreadable, then Nym, then Anyone: one line each");
    /* Direct in between: synced, and the next skip is news again. */
    assert_eq!(step(None, said), Step::Sync);
    assert_eq!(step(nym, None), Step::Skip { why: nym.unwrap_or_default(), log: true });
}

#[test]
fn the_skip_line_is_one_line_naming_the_reason() {
    let why = direct_refused(Some(NYM)).unwrap_or_default();
    let line = skip_line(why);
    assert!(line.starts_with("[NTP] time sync skipped: the chosen network is the Nym mixnet"));
    assert!(line.ends_with("so the RTC time is kept\n"));
    assert_eq!(line.matches('\n').count(), 1);
}
