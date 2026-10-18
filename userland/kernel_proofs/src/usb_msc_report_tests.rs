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


//! The search report driver.usb_msc0 appends to its state reply, written by
//! the driver's own encoder and read by the kernel's words, so the two ends
//! of the wire are held together, as are the generation and the status.

#[path = "usb_msc_class_tests.rs"]
mod class_tests;
#[path = "../../capsule_driver_usb_msc/src/scan/report.rs"]
mod driver_report;
#[path = "usb_msc_generation_tests.rs"]
mod generation_tests;
#[path = "usb_msc_status_tests.rs"]
mod status_tests;

use crate::usb_msc_report_words::{words, REPORT_LEN};
use driver_report::{Report, Stage, REPORT_LEN as DRIVER_LEN};

fn said(r: Report) -> alloc::string::String {
    let mut wire = [0u8; DRIVER_LEN];
    assert_eq!(r.encode(&mut wire), DRIVER_LEN);
    words(&wire)
}

#[test]
fn both_ends_agree_on_the_length() {
    assert_eq!(REPORT_LEN, DRIVER_LEN);
}

#[test]
fn every_stage_is_said_in_words() {
    use Stage::*;
    for stage in [
        NoXhci, NoPort, Busy, Address, Config, NotStorage, Configure, NotReady, Capacity, Bound,
        Gone, NoMedium, Other, Changed,
    ] {
        let line = said(Report::at(stage, 3, -5));
        assert!(line.starts_with("[USB-MSC] "), "{line}");
        assert!(!line.contains("unknown stage"), "{stage:?}: {line}");
        let mut most = Report::at(stage, u8::MAX, i32::MIN);
        (most.connected, most.closed, most.aux, most.blocks) = (255, 255, u32::MAX, u64::MAX);
        assert!(said(most).len() < 200, "one serial line: {}", said(most));
    }
}

#[test]
fn a_stick_that_was_not_ready_names_its_port_its_errno_and_why() {
    let mut r = Report::at(Stage::NotReady, 7, -5);
    (r.connected, r.closed) = (2, 1);
    let line = said(r);
    let why = ["port 7", "errno -5", "a failed check", "three failed requests in a row"];
    assert!(why.iter().all(|w| line.contains(w)), "{line}");
    assert!(line.contains("2 port(s) connected, 1 given up"), "{line}");
}

#[test]
fn a_bound_stick_says_its_size() {
    let line = said(Report { aux: 512, blocks: 60_063_744, ..Report::at(Stage::Bound, 4, 0) });
    assert!(line.contains("bound, 60063744 blocks of 512 bytes"), "{line}");
}
