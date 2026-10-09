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
//! of the wire are held together.

#[path = "../../capsule_driver_usb_msc/src/scan/report.rs"]
mod driver_report;

use crate::usb_msc_report_words::{words, REPORT_LEN};
use driver_report::{first_interface, Report, Stage, REPORT_LEN as DRIVER_LEN};

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
    for stage in [
        Stage::NoXhci,
        Stage::NoPort,
        Stage::Busy,
        Stage::Address,
        Stage::Config,
        Stage::NotStorage,
        Stage::Configure,
        Stage::NotReady,
        Stage::Capacity,
        Stage::Bound,
    ] {
        let line = said(Report::at(stage, 3, -5));
        assert!(line.starts_with("[USB-MSC] "), "{line}");
        assert!(!line.contains("unknown stage"), "{stage:?}: {line}");
        assert!(line.len() < 200, "one serial line: {line}");
    }
}

#[test]
fn a_stick_that_was_not_ready_names_its_port_and_errno() {
    let mut r = Report::at(Stage::NotReady, 7, -5);
    (r.connected, r.closed) = (2, 1);
    let line = said(r);
    assert!(line.contains("port 7") && line.contains("errno -5"), "{line}");
    assert!(line.contains("2 port(s) connected, 1 given up"), "{line}");
}

#[test]
fn a_device_of_another_class_says_its_class() {
    /* A configuration descriptor whose first interface is a HID keyboard. */
    let config = [
        9, 2, 25, 0, 1, 1, 0, 0xa0, 50, //
        9, 4, 0, 0, 1, 3, 1, 1, 0, //
        7, 5, 0x81, 3, 8, 0, 10,
    ];
    let class = first_interface(&config);
    assert_eq!(class, 0x03_01_01);
    let line = said(Report { aux: class, ..Report::at(Stage::NotStorage, 2, 0) });
    assert!(line.contains("class 03, subclass 01, protocol 01"), "{line}");
}

#[test]
fn a_bound_stick_says_its_size() {
    let line = said(Report { aux: 512, blocks: 60_063_744, ..Report::at(Stage::Bound, 4, 0) });
    assert!(line.contains("bound, 60063744 blocks of 512 bytes"), "{line}");
}

#[test]
fn a_truncated_descriptor_names_no_class() {
    assert_eq!(first_interface(&[9, 2, 25]), 0);
    assert_eq!(first_interface(&[]), 0);
}
