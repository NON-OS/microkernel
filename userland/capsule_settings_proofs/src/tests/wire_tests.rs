// NONOS Operating System (AGPL-3.0-or-later)

//! Proofs for the WiFi scan wire format, which parses UNTRUSTED bytes from
//! the WiFi service. A well-formed buffer yields the right networks; a lying
//! count or a truncated entry is refused at the first bad entry without
//! panicking or reading out of bounds.

use super::wire_rig::*;

#[test]
fn a_well_formed_buffer_parses_every_network() {
    let mut buf = vec![2u8];
    buf.extend(entry(200, true, b"home"));
    buf.extend(entry(120, false, b"cafe"));
    let nets = collect(&buf);
    assert_eq!(nets.len(), 2);
    assert_eq!(nets[0].ssid(), b"home");
    assert_eq!(nets[0].signal, 200);
    assert!(nets[0].secured, "flags bit 0 marks secured");
    assert_eq!(nets[1].ssid(), b"cafe");
    assert!(!nets[1].secured);
}

#[test]
fn an_empty_buffer_yields_nothing() {
    assert_eq!(collect(&[]).len(), 0);
    assert_eq!(collect(&[0]).len(), 0, "a count of zero");
}

#[test]
fn a_lying_count_stops_at_the_real_data() {
    // Count claims 5 but only one entry follows.
    let mut buf = vec![5u8];
    buf.extend(entry(90, false, b"one"));
    assert_eq!(collect(&buf).len(), 1, "parsing stops when the data runs out");
}

#[test]
fn a_truncated_entry_header_is_refused() {
    // Count says 1, but only two of the three header bytes are present.
    assert_eq!(collect(&[1, 50, 0]).len(), 0, "a partial entry header is dropped");
}
