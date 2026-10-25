// NONOS Operating System (AGPL-3.0-or-later)

//! Received management frames, cleaned and checked.

use crate::gen3::rx_frame::{parse as parse_frame, DESC_LEN};

// An RX MPDU payload carrying `frame`, with the given flags and status.
fn mpdu(frame: &[u8], flags1: u8, flags2: u8, status: u32) -> Vec<u8> {
    let mut p = vec![0u8; DESC_LEN];
    p[0..2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    p[2] = flags1;
    p[3] = flags2;
    p[12..16].copy_from_slice(&status.to_le_bytes());
    p[40] = 50;
    p[41] = 60;
    p[42] = 11;
    p.extend_from_slice(frame);
    p
}

fn beacon_frame() -> Vec<u8> {
    let mut f = vec![0x80, 0, 0, 0];
    f.extend_from_slice(&[0xFF; 6]);
    f.extend_from_slice(&[2, 1, 1, 1, 1, 1]);
    f.extend_from_slice(&[2, 1, 1, 1, 1, 1]);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&[0xAB; 12]);
    f
}

#[test]
fn received_management_frames_are_cleaned_and_checked() {
    let b = beacon_frame();
    let f = parse_frame(&mpdu(&b, 0, 0, 3)).expect("good beacon");
    assert_eq!(f.frame, b);
    assert_eq!((f.channel, f.signal), (11, -50), "the stronger chain");
    assert!(parse_frame(&mpdu(&b, 0, 0, 2)).is_none(), "bad CRC");
    assert!(parse_frame(&mpdu(&b, 0, 0, 1)).is_none(), "FIFO overrun");
    // Pad after the header, and four bytes of MIC/CRC at the end.
    let mut padded = b[..24].to_vec();
    padded.extend_from_slice(&[0xEE, 0xEE]);
    padded.extend_from_slice(&b[24..]);
    padded.extend_from_slice(&[0xCC; 4]);
    let f = parse_frame(&mpdu(&padded, 0x20, 0x20, 3)).expect("padded");
    assert_eq!(f.frame, b, "pad and tail removed");
    let mut data = b.clone();
    data[0] = 0x08;
    assert!(parse_frame(&mpdu(&data, 0, 0, 3)).is_none(), "data frames are not scan results");
    let mut lying = mpdu(&b, 0, 0, 3);
    lying[0..2].copy_from_slice(&500u16.to_le_bytes());
    assert!(parse_frame(&lying).is_none(), "length past the packet");
    assert!(parse_frame(&mpdu(&b[..20], 0, 0, 3)).is_none(), "shorter than a header");
    assert!(
        parse_frame(&mpdu(&b[..24], 0xF0, 0, 3)).is_some(),
        "a MIC length never cuts the header"
    );
    let whole = mpdu(&b, 0, 0x20, 3);
    for n in 0..whole.len() {
        let _ = parse_frame(&whole[..n]);
    }
}
