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

//! A device reset over and over (review N8). Every stall reset the device
//! and brought it up again, for good: one that never answered was claimed
//! afresh for every request. Now the stall after `RESETS_IN_A_ROW` resets
//! with less than `SERVED_SPAN_MS` served since the last stops the device,
//! gives it back and ends the driver, within a bound; serving the span
//! starts the count over. The device answers as bhyve's legacy one does,
//! with a used length of one, which the legacy rule takes.

use nonos_libc::{bringup::total_sleep_ms, events, now, said, Event, MK_PCI_CMD_MEMORY_SPACE};

use super::served::{claims, up};
use crate::constants::{REQUEST_MS, RESET_MS, VIRTIO_BLK_S_OK};
use crate::server::streak::{RESETS_IN_A_ROW, SERVED_SPAN_MS};

pub const GONE: Option<&str> = Some("mk_exit(6)");

#[test]
fn a_device_that_answers_nothing_is_given_up_within_a_bounded_time() {
    let mut up = up();
    let start = now();
    assert_eq!(up.turns(50).as_deref(), GONE, "reset for good");
    assert_eq!(claims(), 1 + RESETS_IN_A_ROW, "each reset brings the device up again");
    let stalls = u64::from(RESETS_IN_A_ROW) + 1;
    let bound = stalls * (REQUEST_MS + RESET_MS) + u64::from(RESETS_IN_A_ROW) * total_sleep_ms();
    assert!(now() - start <= bound, "{} ms, past {bound}", now() - start);
    let (log, decode) = (events(), Event::Command(MK_PCI_CMD_MEMORY_SPACE));
    assert_eq!(log[log.len() - 2..], [decode, Event::Release], "given back last, unmastered");
    let last = said().pop().unwrap_or_default();
    assert!(last.contains("3 resets in a row with under 60 s served between"), "{last}");
}

#[test]
fn a_device_that_serves_the_span_after_its_resets_starts_the_count_over() {
    let mut up = up();
    assert_eq!(up.turns(RESETS_IN_A_ROW), None);
    up.answers_for(SERVED_SPAN_MS, VIRTIO_BLK_S_OK);
    let before = claims();
    assert_eq!(up.turns(RESETS_IN_A_ROW), None, "given up after serving the span");
    assert_eq!(claims() - before, RESETS_IN_A_ROW, "each stall reset again");
    assert_eq!(up.turns(1).as_deref(), GONE, "the count did not run again");
}
