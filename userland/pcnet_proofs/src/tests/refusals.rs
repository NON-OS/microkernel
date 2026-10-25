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

use super::util::{refused, started};
use crate::model::Event;

// The card never goes on the air under any address when none can be drawn.
#[test]
fn no_entropy_and_the_part_is_never_initialised() {
    nonos_libc::entropy(false);
    let (err, log) = refused(|_| {});
    assert_eq!(err, "no entropy for station address");
    assert!(!log.contains(&Event::Init) && !started(&log));
}

#[test]
fn a_part_that_never_shows_stop_is_refused() {
    let (err, log) = refused(|p| p.faults.stuck_in_reset = true);
    assert!(err.contains("reset"), "{err}");
    assert!(!log.contains(&Event::Bcr(20, 2)), "style written to a part not stopped");
}

#[test]
fn an_id_without_amds_code_is_refused() {
    let (err, log) = refused(|p| p.chip_id = 0x0262_1001);
    assert_eq!(err, "chip id is not an AMD PCnet");
    assert!(!log.contains(&Event::Init));
}

#[test]
fn a_part_that_never_takes_the_init_block_is_refused() {
    let (err, log) = refused(|p| p.faults.deaf_to_init = true);
    assert_eq!(err, "init block not taken (no IDON)");
    assert!(!started(&log));
}

#[test]
fn a_part_that_never_starts_is_refused() {
    let (err, _) = refused(|p| p.faults.never_starts = true);
    assert_eq!(err, "part did not start");
}
