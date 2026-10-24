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
//! one per field, and a refused field stops nothing.

use super::hydrate_rig::*;
use crate::ipc::error::IpcError;

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
