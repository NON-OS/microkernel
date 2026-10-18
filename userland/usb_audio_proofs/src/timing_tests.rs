// NONOS Operating System (AGPL-3.0-or-later)
//! How the stream is cut into packets and batches.

use crate::uac::{timing, Timing};

const FULL: u8 = 1;
const HIGH: u8 = 3;

#[test]
fn full_speed_sends_one_millisecond_a_packet_and_eight_a_batch() {
    assert_eq!(timing(FULL, 1, 192, 2), Some(Timing { packet_bytes: 192, packets: 8 }));
    assert_eq!(timing(FULL, 1, 192, 2).map(|t| t.batch_bytes()), Some(1536));
    // A full-speed bInterval of 2 is 2^(2-1) frames: 2 ms.
    assert_eq!(timing(FULL, 2, 384, 2), Some(Timing { packet_bytes: 384, packets: 4 }));
}

#[test]
fn high_speed_counts_microframes() {
    assert_eq!(timing(HIGH, 1, 192, 2), Some(Timing { packet_bytes: 24, packets: 31 }));
    assert_eq!(timing(HIGH, 4, 192, 2), Some(Timing { packet_bytes: 192, packets: 8 }));
    // SuperSpeed ids also count 125 us, with no extra transactions.
    assert_eq!(timing(4, 4, 192, 2), Some(Timing { packet_bytes: 192, packets: 8 }));
}

#[test]
fn a_packet_larger_than_the_endpoint_is_refused() {
    assert_eq!(timing(FULL, 1, 191, 2), None);
    assert_eq!(timing(HIGH, 4, 100, 2), None);
    // Two transactions a microframe (bits 12:11 = 1) carry 200 bytes.
    let two = 1 << 11 | 100;
    assert_eq!(timing(HIGH, 4, two, 2), Some(Timing { packet_bytes: 192, packets: 8 }));
    // Those bits mean nothing at full speed.
    assert_eq!(timing(FULL, 1, 1 << 11 | 100, 2), None);
}

#[test]
fn a_batch_fits_one_isoch_request() {
    // The largest endpoint each speed may have: 1023 bytes, and 1024 with
    // two transactions added.
    for (speed, most) in [(FULL, 0x03FF), (HIGH, 0x1400)] {
        for interval in 1..=16 {
            if let Some(t) = timing(speed, interval, most, 2) {
                assert!(t.batch_bytes() <= 4032 && t.packets as usize <= 31 && t.packets > 0);
            }
        }
    }
    // 32 ms of sound a packet: one packet a batch, and still too large.
    assert_eq!(timing(FULL, 6, 1023, 2), None);
}

#[test]
fn a_batch_is_never_more_packets_than_driver_xhci0_takes() {
    // Two batches share a 63-TRB ring (the the review): xhci's
    // ISOCH_PACKETS_MAX is 31, and every batch this driver builds fits it.
    let xhci = include_str!("../../capsule_driver_xhci/src/protocol/limits.rs");
    let max: usize = xhci
        .lines()
        .find_map(|l| l.strip_prefix("pub const ISOCH_PACKETS_MAX: usize = "))
        .and_then(|v| v.trim_end_matches(';').parse().ok())
        .expect("xhci names its batch limit");
    assert!(2 * max < 64, "two batches overrun the ring");
    for interval in 1..=4u8 {
        if let Some(t) = timing(HIGH, interval, 1024, 2) {
            assert!(t.packets as usize <= max, "interval {interval}: {}", t.packets);
        }
    }
}

#[test]
fn one_channel_moves_half_the_bytes() {
    assert_eq!(timing(FULL, 1, 96, 1), Some(Timing { packet_bytes: 96, packets: 8 }));
    assert_eq!(timing(FULL, 1, 96, 2), None);
    assert_eq!(timing(FULL, 1, 192, 0), None, "no channels, no stream");
}
