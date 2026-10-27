// NONOS Operating System (AGPL-3.0-or-later)
//! Frames up from well-formed transfers: each in order, a request at a
//! time, without its pad; what the chip marked bad, too short or too long
//! counted and let go; the next transfer read only once this one is out.

use nonos_usbnet::Nic;

use crate::aggregate::{aggregate, frame, Frame, CRC_ERROR, DROPPED};
use crate::ax::link::RX_LEN;
use crate::harness::bound;
use crate::seen::{drain, feed};

#[test]
fn an_aggregate_comes_up_frame_by_frame_in_order() {
    for fillers in [false, true] {
        let (bus, mut ax, _) = bound();
        let frames: Vec<Frame> = (0..5).map(|n| (frame(n, 60 + 97 * n as usize), 0)).collect();
        feed(&bus, aggregate(&frames, fillers));
        let want: Vec<Vec<u8>> = frames.into_iter().map(|f| f.0).collect();
        assert_eq!(drain(&mut ax), want);
    }
}

#[test]
fn the_next_transfer_waits_for_every_frame_of_this_one() {
    let (bus, mut ax, _) = bound();
    feed(&bus, aggregate(&[(frame(1, 64), 0), (frame(2, 64), 0)], false));
    feed(&bus, aggregate(&[(frame(3, 64), 0)], false));
    let mut out = [0u8; 1514];
    assert_eq!(ax.recv(&mut out), Ok(Some(64)));
    assert_eq!(bus.0.borrow().bulk_in.len(), 1, "the second transfer was read early");
    assert_eq!(ax.recv(&mut out), Ok(Some(64)));
    assert_eq!((ax.recv(&mut out), out[0]), (Ok(Some(64)), 3u8.wrapping_mul(31)));
}

#[test]
fn marked_short_and_long_frames_are_counted_and_let_go() {
    let (bus, mut ax, _) = bound();
    let bad = [
        (frame(1, 100), CRC_ERROR),
        (frame(2, 100), DROPPED),
        (frame(3, 13), 0),
        (frame(4, 1514 + 5), 0),
    ];
    feed(&bus, aggregate(&bad, false));
    feed(&bus, aggregate(&[(frame(5, 1514 + 4), 0), (frame(6, 1514), 0)], false));
    let got = drain(&mut ax);
    assert!(got == [frame(5, 1514), frame(6, 1514)], "an FCS cut, a full frame whole");
    assert_eq!((ax.counts.errors, ax.counts.too_long), (3, 1));
}

#[test]
fn a_transfer_that_fills_the_buffer_is_let_go_with_its_rest() {
    let (bus, mut ax, _) = bound();
    let whole = aggregate(&[(frame(1, 64), 0)], false);
    let mut full = vec![0u8; RX_LEN];
    full[RX_LEN - whole.len()..].copy_from_slice(&whole);
    feed(&bus, full);
    feed(&bus, whole.clone());
    feed(&bus, whole);
    let got = drain(&mut ax);
    assert_eq!((got.len(), ax.counts.overruns), (1, 1), "only the transfer after the rest");
}
