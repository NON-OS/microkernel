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

//! The pass that reads every stored value when the settings panel opens. A
//! policy service that is registered but silent costs one reply timeout, not
//! one per field, and a pass that missed values reports it, so defaults on
//! screen never pass for stored values.

use crate::ipc::error::IpcError;
use crate::ipc::hydrate_pass::{hydrate_fields, hydrate_final, hydrate_report, HYDRATE_RETRY_MS};

/// The status strip's buffer (`Status::MAX_MSG` in the capsule).
const STATUS_CAP: usize = 96;

/// Run the pass over `fields` with `answer` standing in for the policy
/// service; return what it stored, how many reads it made and what it reported.
fn run(
    fields: &[u32],
    answer: impl Fn(u32) -> Result<u32, IpcError>,
) -> (Vec<(u32, u32)>, usize, Option<IpcError>) {
    let mut stored = Vec::new();
    let mut reads = 0;
    let get = |field| {
        reads += 1;
        answer(field)
    };
    let err = hydrate_fields(fields, get, |field, value| stored.push((field, value)));
    (stored, reads, err)
}

#[test]
fn a_service_that_answers_everything_fills_every_field_and_reports_nothing() {
    let (stored, reads, err) = run(&[1, 2, 3], |f| Ok(f * 10));
    assert_eq!(stored, vec![(1, 10), (2, 20), (3, 30)]);
    assert_eq!(reads, 3);
    assert!(err.is_none());
}

#[test]
fn a_silent_service_costs_one_timeout_not_one_per_field() {
    let fields: Vec<u32> = (0..16).collect();
    let (stored, reads, err) = run(&fields, |_| Err(IpcError::RecvTimeout));
    assert_eq!(reads, 1);
    assert!(stored.is_empty());
    assert!(matches!(err, Some(IpcError::RecvTimeout)));
}

#[test]
fn a_service_that_goes_silent_midway_keeps_what_it_answered() {
    let (stored, reads, err) =
        run(&[1, 2, 3, 4], |f| if f < 3 { Ok(f) } else { Err(IpcError::RecvTimeout) });
    assert_eq!(stored, vec![(1, 1), (2, 2)]);
    assert_eq!(reads, 3);
    assert!(matches!(err, Some(IpcError::RecvTimeout)));
}

#[test]
fn a_refused_field_does_not_stop_the_rest_and_is_reported() {
    let (stored, reads, err) =
        run(&[1, 2, 3], |f| if f == 2 { Err(IpcError::Status(7)) } else { Ok(f) });
    assert_eq!(stored, vec![(1, 1), (3, 3)]);
    assert_eq!(reads, 3);
    assert!(matches!(err, Some(IpcError::Status(7))));
}

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

/* Soon enough to replace the defaults while the window is still being read. */
const _: () = assert!(HYDRATE_RETRY_MS > 0 && HYDRATE_RETRY_MS <= 5_000);
