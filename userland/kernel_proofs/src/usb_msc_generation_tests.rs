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

//! The re-bind generation that ends driver.usb_msc0's state reply, as the
//! kernel reads it (src/hardware/usb_msc_capsule/generation.rs): after the
//! counters and the driver's report in a reply of today, and 0 from a driver
//! that predates it, whose reply ends at the report.

#[path = "../../../src/hardware/usb_msc_capsule/generation.rs"]
mod generation;

use super::driver_report::REPORT_LEN as DRIVER_LEN;
/* generation.rs names the kernel's words as its sibling. */
use crate::usb_msc_report_words as report_words;
use generation::{generation, COUNTERS_LEN};

#[test]
fn a_reply_of_today_carries_the_generation_after_the_report() {
    let mut body = vec![0u8; COUNTERS_LEN + DRIVER_LEN];
    body.extend_from_slice(&7u32.to_le_bytes());
    assert_eq!(generation(&body), Some(7));
    body.extend_from_slice(&[0xEE; 8]);
    assert_eq!(generation(&body), Some(7), "what a later driver adds after it is not read");
    body[COUNTERS_LEN + DRIVER_LEN..][..4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(generation(&body), Some(u32::MAX));
}

#[test]
fn a_reply_that_predates_it_reads_zero_and_a_short_one_nothing() {
    let old = COUNTERS_LEN + DRIVER_LEN;
    assert_eq!(generation(&vec![0; old]), Some(0));
    for len in [0, 1, COUNTERS_LEN, old - 1, old + 1, old + 3] {
        assert_eq!(generation(&vec![0xA5; len]), None, "{len} bytes");
    }
}
