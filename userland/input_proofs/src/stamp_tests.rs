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

//! Every event the router takes from the kernel ring leaves with a time:
//! the ring hands them all with none, and the Terminal and the Text Editor
//! time a double click by it, so two presses on the same spot minutes apart
//! were taken for one double click.

use nonos_libc::InputEvent;

use crate::libc_frame::delivery::decode_delivery;
use crate::protocol::{encode_delivery, DELIVERY_LEN};
use crate::router_stamp::stamp;

const BUTTON_DOWN: u16 = 5;
/// The Terminal's and the Text Editor's double-click span.
const MULTI_CLICK_NS: u64 = 400_000_000;

fn press(x: i32, y: i32) -> InputEvent {
    InputEvent { kind: BUTTON_DOWN, code: 1, x, y, ..InputEvent::default() }
}

/// The event as the receiver reads it out of the router's frame.
fn delivered(ev: &InputEvent) -> InputEvent {
    let mut frame = [0u8; DELIVERY_LEN];
    encode_delivery(&mut frame, ev, Some(7));
    decode_delivery(&frame).expect("a whole frame").event
}

#[test]
fn a_batch_with_no_time_takes_the_drain_time_in_nanoseconds() {
    let mut batch = [press(10, 10), press(20, 20), press(30, 30)];
    stamp(&mut batch, 12_345);
    assert!(batch.iter().all(|e| e.timestamp_ns == 12_345_000_000));
    assert_eq!(batch[1].x, 20, "nothing else of the event changes");
}

#[test]
fn a_time_the_source_gave_is_kept() {
    let mut batch = [InputEvent { timestamp_ns: 77, ..press(1, 1) }, press(2, 2)];
    stamp(&mut batch, 5);
    assert_eq!(batch[0].timestamp_ns, 77);
    assert_eq!(batch[1].timestamp_ns, 5_000_000);
}

#[test]
fn a_clock_below_zero_or_past_nanoseconds_leaves_the_time_unknown() {
    for now in [-1, i64::MIN, i64::MAX] {
        let mut batch = [press(1, 1)];
        stamp(&mut batch, now);
        assert_eq!(batch[0].timestamp_ns, 0, "{now}");
    }
    let mut none: [InputEvent; 0] = [];
    stamp(&mut none, 9);
}

#[test]
fn the_time_reaches_the_window_in_the_delivery_frame() {
    let mut batch = [press(40, 50)];
    stamp(&mut batch, 2_000);
    assert_eq!(delivered(&batch[0]).timestamp_ns, 2_000_000_000);
}

#[test]
fn presses_on_one_spot_two_seconds_apart_are_not_one_double_click() {
    let (mut first, mut second) = ([press(40, 50)], [press(40, 50)]);
    stamp(&mut first, 10_000);
    stamp(&mut second, 12_000);
    let gap = |a: &InputEvent, b: &InputEvent| b.timestamp_ns.saturating_sub(a.timestamp_ns);
    let (a, b) = (delivered(&first[0]), delivered(&second[0]));
    assert!(gap(&a, &b) >= MULTI_CLICK_NS, "2 s apart is no double click");
    // Before, both reached the window as time 0, and 0 - 0 is inside the span.
    let (a0, b0) = (delivered(&press(40, 50)), delivered(&press(40, 50)));
    assert!(gap(&a0, &b0) < MULTI_CLICK_NS);
    let mut quick = [press(40, 50)];
    stamp(&mut quick, 10_250);
    assert!(gap(&a, &delivered(&quick[0])) < MULTI_CLICK_NS, "250 ms apart still is");
}
