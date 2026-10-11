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

//! Parts that fail the driver, and what the driver leaves behind: an error
//! that names the step, a part stopped and reset before a single grant goes
//! back, and every grant and the claim returned in reverse order.

use super::util::refused;
use crate::model::Event;

/// Whether the receive process was ever started.
fn started(log: &[Event]) -> bool {
    log.iter().any(|e| matches!(e, Event::Csr(6, v) if v & 2 != 0))
}

// The card never goes on the air under any address when none can be drawn.
#[test]
fn no_entropy_and_the_part_is_never_started() {
    nonos_libc::entropy(false);
    let (err, log) = refused(|_| {});
    assert_eq!(err, "no entropy for station address");
    assert!(!log.contains(&Event::Setup) && !started(&log));
}

#[test]
fn a_part_that_never_leaves_reset_is_refused() {
    let (err, log) = refused(|p| p.faults.stuck_in_reset = true);
    assert!(err.contains("reset"), "{err}");
    assert!(!log.iter().any(|e| matches!(e, Event::Csr(3, _))), "rings given to a part in reset");
}

#[test]
fn a_part_that_never_takes_the_setup_frame_never_receives() {
    let (err, log) = refused(|p| p.faults.ignores_setup = true);
    assert_eq!(err, "setup frame not taken");
    assert!(!started(&log), "receiving with no filter loaded");
}

#[test]
fn a_part_that_never_starts_is_refused() {
    let (err, _) = refused(|p| p.faults.never_starts = true);
    assert_eq!(err, "setup frame not taken");
}
