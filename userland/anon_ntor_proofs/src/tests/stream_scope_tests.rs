// NONOS Operating System (AGPL-3.0-or-later)
//! A relay cell reaches only streams on the circuit it arrived on, and a
//! stream-level SENDME refills that stream's window.

use crate::stream::{find_on, Stream, StreamStage, REASON_DESTROY};

#[test]
fn a_cell_on_one_circuit_cannot_reach_a_stream_on_another() {
    // Ids are only unique per circuit in the protocol: stream 7 on circuit 1
    // and stream 7 on circuit 2 are different connections.
    let mut streams = vec![Stream::new(7, 1), Stream::new(7, 2)];
    find_on(&mut streams, 2, 7).unwrap().inbound.extend_from_slice(b"for circuit two");
    assert!(streams[0].inbound.is_empty(), "circuit 1's stream got circuit 2's payload");
    assert_eq!(streams[1].inbound, b"for circuit two");
    assert!(find_on(&mut streams, 3, 7).is_none());
}

#[test]
fn a_stream_sendme_refills_the_stream_window() {
    let mut s = Stream::new(1, 1);
    let start = s.package_window;
    for _ in 0..start {
        s.package_window -= 1;
    }
    assert_eq!(s.package_window, 0, "a stream stops at 500 cells without a grant");
    s.credit_package();
    assert_eq!(s.package_window, 50);
}

#[test]
fn a_dead_circuit_ends_its_open_streams_but_keeps_an_earlier_reason() {
    let mut open = Stream::new(1, 1);
    open.stage = StreamStage::Open;
    open.end_with_circuit();
    assert!(matches!(open.stage, StreamStage::Ended(REASON_DESTROY)));
    let mut done = Stream::new(2, 1);
    done.stage = StreamStage::Ended(6);
    done.end_with_circuit();
    assert!(matches!(done.stage, StreamStage::Ended(6)));
}
