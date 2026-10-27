// NONOS Operating System (AGPL-3.0-or-later)
//! The receive descriptor walk on transfers built by hand: a frame, frames
//! gathered and aligned, and each way a descriptor can lie.

use crate::rtl::rx::{next, Next, DESC};

/// A descriptor for a packet of `len` bytes with its FCS, `high` set in
/// the bits above the length, then the packet.
pub fn packet(len: usize, high: u32, fill: u8) -> Vec<u8> {
    let mut v = vec![0u8; DESC + len];
    v[..4].copy_from_slice(&(len as u32 | high).to_le_bytes());
    v[4..24].fill(0xa5); // checksum and VLAN fields, never read
    v[DESC..].fill(fill);
    v
}

#[test]
fn one_packet_gives_its_frame_without_the_fcs() {
    let t = packet(64, 0, 0x11);
    assert_eq!(next(&t, 0), Next::Frame { start: 24, len: 60, next: 88 });
    assert_eq!(next(&t, 88), Next::End);
    let t = packet(1518, 0, 0x22);
    assert_eq!(next(&t, 0), Next::Frame { start: 24, len: 1514, next: 1544 });
}

#[test]
fn gathered_packets_are_found_at_each_multiple_of_eight() {
    let mut t = packet(65, 0, 1);
    t.resize(96, 0xee); // 24 + 65 = 89, the next descriptor at 96
    t.extend(packet(100, 0, 2));
    assert_eq!(next(&t, 0), Next::Frame { start: 24, len: 61, next: 96 });
    assert_eq!(next(&t, 96), Next::Frame { start: 120, len: 96, next: 224 });
    assert_eq!(next(&t, 224), Next::End);
}

#[test]
fn bits_above_the_length_are_not_length() {
    let t = packet(64, 0xffff_8000, 3);
    assert_eq!(next(&t, 0), Next::Frame { start: 24, len: 60, next: 88 });
}

#[test]
fn a_packet_shorter_than_any_frame_breaks_the_walk() {
    for len in [0, 1, 4, 18, 59] {
        let mut t = packet(len, 0, 4);
        t.resize(200, 0);
        assert_eq!(next(&t, 0), Next::Broken, "{len}");
    }
}

#[test]
fn a_packet_past_the_bytes_that_came_breaks_the_walk() {
    let t = packet(64, 0, 5);
    assert_eq!(next(&t[..87], 0), Next::Broken);
    let mut lying = packet(64, 0, 6);
    lying[..4].copy_from_slice(&0x7fffu32.to_le_bytes());
    assert_eq!(next(&lying, 0), Next::Broken);
}

#[test]
fn too_few_bytes_for_a_descriptor_end_the_transfer() {
    let t = packet(64, 0, 7);
    for n in 0..=DESC {
        assert_eq!(next(&t[..n], 0), Next::End, "{n}");
    }
    assert_eq!(next(&t, 4096), Next::End, "a cursor past the end");
}
