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

//! Nothing a transfer carried stays in this driver once it is answered
//!. The server reuses one reply buffer for every reply, and
//! the HID driver's own clears do not reach it, so the last input report
//! would sit there until a longer reply covered it. These run the shipping
//! handlers on a buffer like the server's, then look in it.

use nonos_i2cmodel::touchpad::{touch_report, Touch};
use nonos_i2cmodel::Config;
use nonos_libc::take_reply;

use super::fixture::{bench, bench_with, pad, MAX_INPUT, PAD};
use super::kept::{answered, body, reply_buffer, wiped, INPUT};
use crate::protocol::{Request, OP_PROBE, TRANSFER_READ_MAX};
use crate::server::handlers::probe;

#[test]
fn an_input_report_answered_leaves_none_of_it_in_the_reply_buffer() {
    let b = bench(Config::LPSS);
    let report = touch_report(&Touch::finger(0x1234, 0x0567));
    b.pad.lock().push_input(&report);
    let (sent, kept) = answered(&b, &body(PAD, &INPUT, usize::from(MAX_INPUT)));
    assert!(sent.windows(report.len()).any(|w| w == report), "the report was not delivered");
    assert!(wiped(&kept), "the reply buffer still holds the report: {kept:02x?}");
}

#[test]
fn the_longest_read_leaves_nothing_and_one_past_it_is_refused_and_leaves_nothing() {
    let b = bench(Config::LPSS);
    b.pad.lock().push_input(&touch_report(&Touch::finger(0x0FED, 0x0CBA)));
    let (sent, kept) = answered(&b, &body(PAD, &INPUT, TRANSFER_READ_MAX));
    assert!(sent.len() > TRANSFER_READ_MAX, "the longest read was not answered whole");
    assert!(wiped(&kept), "the longest read left bytes behind");
    let (_, kept) = answered(&b, &body(PAD, &INPUT, TRANSFER_READ_MAX + 1));
    assert!(wiped(&kept), "the refusal left its reply behind");
}

#[test]
fn an_error_a_refusal_and_a_probe_leave_no_reply_behind() {
    let deaf = bench_with(Config::LPSS, pad().deaf());
    assert!(wiped(&answered(&deaf, &body(PAD, &INPUT, 2)).1), "the NACK's reply was kept");
    let b = bench(Config::LPSS);
    assert!(wiped(&answered(&b, &body(0x2C, &INPUT, 2)).1), "the refusal's reply was kept");
    let mut out = reply_buffer();
    probe::handle(&b.driver, 42, &Request { op: OP_PROBE, request_id: 8 }, &[PAD], &mut out);
    assert!(take_reply().is_some() && wiped(&out), "the probe's reply was kept");
}
