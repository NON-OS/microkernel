//! A receive batch is read whole or not at all.

use crate::batch_frames::batch_frames;

/// A body the way the driver writes one: count, then length and frame.
fn body(frames: &[&[u8]]) -> Vec<u8> {
    let mut out = (frames.len() as u32).to_le_bytes().to_vec();
    for f in frames {
        out.extend_from_slice(&(f.len() as u32).to_le_bytes());
        out.extend_from_slice(f);
    }
    out
}

#[test]
fn every_frame_comes_back_in_order() {
    let a = vec![0xa1u8; 1514];
    let b = vec![0xb2u8; 60];
    let got = batch_frames(&body(&[&a, &b, &[]])).expect("a well formed batch reads");
    assert_eq!(got, vec![a, b, vec![]]);
}

#[test]
fn a_full_batch_of_44_largest_frames_reads() {
    let f = vec![7u8; 1514];
    let frames: Vec<&[u8]> = (0..44).map(|_| &f[..]).collect();
    assert_eq!(batch_frames(&body(&frames)).map(|v| v.len()), Some(44));
}

#[test]
fn a_count_larger_than_the_frames_present_is_refused() {
    let mut b = body(&[b"one", b"two"]);
    b[..4].copy_from_slice(&3u32.to_le_bytes());
    assert_eq!(batch_frames(&b), None);
}

#[test]
fn trailing_bytes_after_the_last_frame_are_refused() {
    let mut b = body(&[b"one"]);
    b.push(0);
    assert_eq!(batch_frames(&b), None);
}

#[test]
fn a_length_running_past_the_end_is_refused() {
    let mut b = body(&[b"frame"]);
    b[4..8].copy_from_slice(&6u32.to_le_bytes());
    assert_eq!(batch_frames(&b), None);
    assert_eq!(batch_frames(&[1, 0]), None, "no room for the count");
}
