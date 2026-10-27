// NONOS Operating System (AGPL-3.0-or-later)
//! Transfers whose trailer or table lies: nothing is read past the bytes
//! that came, a lie in the trailer drops the transfer whole, an entry past
//! the frames ends the table, and no transfer gives more than SPANS_MAX.

use crate::aggregate::{aggregate, frame, Frame};
use crate::ax::link::SPANS_MAX;
use crate::ax::rx_header::parse;
use crate::harness::bound;
use nonos_usbnet::Nic;

fn trailer(count: u32, table: u32) -> [u8; 4] {
    (count | table << 16).to_le_bytes()
}

fn spans(t: &[u8]) -> (usize, crate::ax::rx_span::Tally) {
    let mut out = vec![];
    let tally = parse(t, &mut out, SPANS_MAX);
    (out.len(), tally)
}

#[test]
fn a_trailer_naming_bytes_that_never_came_drops_the_transfer() {
    for t in [vec![], vec![1, 0, 0], vec![0; 3]] {
        assert!(spans(&t).1.lie, "{} bytes", t.len());
    }
    let mut t = vec![0u8; 64];
    for (count, table) in [(1, 61), (16, 0), (0xFFFF, 0), (1, 0xFFFF), (15, 1)] {
        t[60..].copy_from_slice(&trailer(count, table));
        assert!(spans(&t).1.lie, "count {count} table {table}");
    }
    t[60..].copy_from_slice(&trailer(15, 0));
    assert!(!spans(&t).1.lie, "a table that just fits is read");
}

#[test]
fn an_entry_longer_than_the_frames_before_the_table_ends_the_table() {
    let mut t = aggregate(&[(frame(1, 60), 0), (frame(2, 60), 0)], false);
    let at = u16::from_le_bytes([t[t.len() - 2], t[t.len() - 1]]) as usize;
    // The second entry claims the longest length the field holds.
    t[at + 4..at + 8].copy_from_slice(&(0x1FFFu32 << 16).to_le_bytes());
    let (n, tally) = spans(&t);
    assert_eq!((n, tally.beyond, tally.lie), (1, true, false));
}

#[test]
fn no_transfer_gives_more_than_spans_max_frames() {
    let frames: Vec<Frame> = (0..SPANS_MAX as u8 + 6).map(|n| (frame(n, 14), 0)).collect();
    let t = aggregate(&frames, false);
    assert!(t.len() < 4096);
    let (n, tally) = spans(&t);
    assert_eq!((n, tally.over), (SPANS_MAX, 6));
}

#[test]
fn a_frame_longer_than_the_room_given_is_counted_and_the_next_still_comes() {
    let (bus, mut ax, _) = bound();
    let t = aggregate(&[(frame(1, 600), 0), (frame(2, 60), 0)], false);
    bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t)));
    let mut small = [0u8; 100];
    assert_eq!(ax.recv(&mut small), Ok(None));
    assert_eq!((ax.recv(&mut small), ax.counts.too_long), (Ok(Some(60)), 1));
}
