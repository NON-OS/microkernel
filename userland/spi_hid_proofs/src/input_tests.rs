// NONOS Operating System (AGPL-3.0-or-later)
//! Input reports: a header taken only with its version and sync byte, its
//! length in words; a body taken only whole; the device descriptor only at
//! its own length.

use nonos_spi_hid::{body, descriptor, header, Header, HEADER_SYNC, IN_DATA};

fn hdr(words: u32, last: bool) -> [u8; 4] {
    let mut h = 3 | words << 8 | if last { 1 << 22 } else { 0 };
    h = h & 0x00FF_FFFF | (HEADER_SYNC as u32) << 24;
    h.to_le_bytes()
}

#[test]
fn a_header_gives_its_body_length_in_words() {
    assert_eq!(header(hdr(5, true)), Some(Header { body_len: 20, last: true }));
    assert_eq!(header(hdr(0x3FFF, false)).map(|h| h.body_len), Some(0x3FFF * 4));
    let mut wrong = hdr(5, true);
    wrong[3] = 0;
    assert_eq!(header(wrong), None, "no sync byte");
    let mut v2 = hdr(5, true);
    v2[0] = v2[0] & 0xF0 | 2;
    assert_eq!(header(v2), None, "another version");
}

#[test]
fn a_body_is_taken_whole_or_not_at_all() {
    let b = [IN_DATA, 3, 0, 7, 0xAA, 0xBB, 0xCC, 0, 0];
    let got = body(&b).unwrap();
    assert_eq!((got.kind, got.id, got.content), (IN_DATA, 7, &[0xAA, 0xBB, 0xCC][..]));
    assert_eq!(body(&b[..6]), None, "content cut short");
    assert_eq!(body(&[IN_DATA, 0xFF, 0xFF, 0]), None, "a length past the body");
}

#[test]
fn the_device_descriptor_says_its_own_length() {
    let mut d = [0u8; 24];
    for (i, w) in [24u16, 0x0300, 512, 64, 64, 256, 0x1234, 0x5678].iter().enumerate() {
        d[2 * i..2 * i + 2].copy_from_slice(&w.to_le_bytes());
    }
    let got = descriptor(&d).unwrap();
    assert_eq!(
        (got.report_desc_len, got.max_input, got.vendor, got.product),
        (512, 64, 0x1234, 0x5678)
    );
    d[0] = 23;
    assert_eq!(descriptor(&d), None);
    assert_eq!(descriptor(&[24, 0]), None, "cut short");
}
