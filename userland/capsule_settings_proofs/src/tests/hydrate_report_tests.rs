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

//! A pass that missed values reports it, so defaults on screen never pass
//! for stored values, and only a silent pass is run again.

use super::hydrate_rig::*;
use crate::ipc::error::IpcError;
use crate::ipc::hydrate_pass::{hydrate_final, hydrate_report};

#[test]
fn the_first_failure_is_the_one_reported() {
    let first = |f| if f == 1 { IpcError::ShortReply } else { IpcError::KindMismatch };
    let (_, _, err) = run(&[1, 2], |f| Err(first(f)));
    assert!(matches!(err, Some(IpcError::ShortReply)));
}

#[test]
fn the_report_says_the_values_on_screen_are_not_stored() {
    let silent = hydrate_report(IpcError::RecvTimeout);
    assert!(silent.starts_with(b"Policy service did not answer"));
    assert!(silent.ends_with(b"values shown are not stored"));
    for err in [IpcError::ShortReply, IpcError::BadHeader, IpcError::Status(1)] {
        assert!(hydrate_report(err).ends_with(b"those shown are defaults"));
    }
}

#[test]
fn every_report_fits_the_status_strip_whole() {
    for err in [IpcError::RecvTimeout, IpcError::KindMismatch, IpcError::SendFailed] {
        assert!(hydrate_report(err).len() <= STATUS_CAP);
    }
}

/* A window opened while the policy store was silent showed defaults for as
 * long as it stayed open: only silence is tried again, and soon. */
#[test]
fn only_a_silent_pass_is_run_again() {
    assert!(!hydrate_final(Some(IpcError::RecvTimeout)));
    assert!(hydrate_final(None));
    for err in [IpcError::ShortReply, IpcError::BadHeader, IpcError::Status(1)] {
        assert!(hydrate_final(Some(err)), "an answered error will answer the same way");
    }
}
