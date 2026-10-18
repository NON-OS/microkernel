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

//! The search for a pad ends by elapsed time, thirty seconds
//! from the first probe however often it asks, and says why it ended even
//! after it said it was retrying.

use nonos_i2cmodel::touchpad::report_descriptor;
use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::{mk_idle_ms, mk_uptime_ms, take_debug};

use super::fixture::{rig_with, HID_DESC_REG, MAX_INPUT, PAD};
use super::recorder::recorded;
use crate::setup;

#[test]
fn a_bus_with_no_pad_is_given_up_after_thirty_seconds_not_a_count_of_probes() {
    let stray = HidOverI2c::new(0x3A, report_descriptor(), MAX_INPUT, 0x04F3, 0x3028);
    let r = recorded(stray, None, None);
    let start = mk_uptime_ms();
    let mut state = setup::run().expect("setup");
    (0..200).for_each(|_| setup::reprobe(&mut state));
    assert!(!state.done, "given up after {} fast probes, in well under 30 s", state.probes);
    mk_idle_ms((start + 29_000 - mk_uptime_ms()).max(0) as u64);
    setup::reprobe(&mut state);
    assert!(!state.done, "given up before thirty seconds");
    mk_idle_ms(1_500);
    setup::reprobe(&mut state);
    assert!(state.done, "still looking after thirty seconds");
    let asked = r.calls.borrow().len();
    setup::reprobe(&mut state);
    assert_eq!(r.calls.borrow().len(), asked, "asked again after giving up");
}

#[test]
fn a_pad_whose_report_descriptor_never_reads_is_given_up_with_its_reason_and_put_to_sleep() {
    // wReportDescLength 1100 is past what setup reads.
    let device = HidOverI2c::new(PAD, vec![0x05; 1100], MAX_INPUT, 0x04F3, 0x3028);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let mut state = setup::run().expect("setup");
    (0..100).for_each(|_| setup::reprobe(&mut state));
    mk_idle_ms(31_000);
    setup::reprobe(&mut state);
    assert!(state.done && !state.found());
    let lines = take_debug();
    let said = |what: &str| lines.iter().filter(|l| l.contains(what)).count();
    assert_eq!(said("retrying"), 1, "the retrying line, once: {lines:?}");
    assert_eq!(said("came up"), 1, "the reason the driver left: {lines:?}");
    let last = r.pad.lock().take_commands().pop();
    assert_eq!(last, Some(Command::SetPower { sleep: true }), "left powered on");
}
