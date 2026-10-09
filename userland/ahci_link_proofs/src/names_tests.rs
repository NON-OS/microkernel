// NONOS Operating System (AGPL-3.0-or-later)
//! The model and serial the driver reads from IDENTIFY: ATA strings, two
//! characters to a word, first character in the high byte, space padded.

use crate::constants::identify::{IDENTIFY_WORDS, MODEL_BYTES, SERIAL_BYTES, W_MODEL, W_SERIAL};
use crate::identity::names;

/// Put `text` at `word` as an ATA string of `bytes`, padded with spaces.
fn put(w: &mut [u16; IDENTIFY_WORDS], word: usize, bytes: usize, text: &[u8]) {
    let mut raw = vec![b' '; bytes];
    raw[..text.len()].copy_from_slice(text);
    for (i, pair) in raw.chunks(2).enumerate() {
        w[word + i] = u16::from(pair[0]) << 8 | u16::from(pair[1]);
    }
}

#[test]
fn a_real_drive_reads_back_as_written() {
    let mut w = [0u16; IDENTIFY_WORDS];
    put(&mut w, W_MODEL, MODEL_BYTES, b"ST1000LM035-1RK172");
    put(&mut w, W_SERIAL, SERIAL_BYTES, b"      WL1A2B3C");
    let n = names(&w);
    assert_eq!(n.model(), b"ST1000LM035-1RK172");
    assert_eq!(n.serial(), b"WL1A2B3C", "leading pad trimmed");
    // QEMU's own.
    put(&mut w, W_MODEL, MODEL_BYTES, b"QEMU HARDDISK");
    put(&mut w, W_SERIAL, SERIAL_BYTES, b"QM00001");
    let n = names(&w);
    assert_eq!((n.model(), n.serial()), (&b"QEMU HARDDISK"[..], &b"QM00001"[..]));
}

#[test]
fn a_full_field_keeps_every_byte() {
    let mut w = [0u16; IDENTIFY_WORDS];
    let model = [b'M'; MODEL_BYTES];
    let serial = [b'7'; SERIAL_BYTES];
    put(&mut w, W_MODEL, MODEL_BYTES, &model);
    put(&mut w, W_SERIAL, SERIAL_BYTES, &serial);
    let n = names(&w);
    assert_eq!(n.model(), &model[..]);
    assert_eq!(n.serial(), &serial[..]);
}

#[test]
fn an_empty_or_hostile_block_gives_printable_text_only() {
    let n = names(&[0u16; IDENTIFY_WORDS]);
    assert_eq!((n.model_len, n.serial_len), (0, 0));
    let n = names(&[0xffff; IDENTIFY_WORDS]);
    assert_eq!(n.model(), &[b'?'; MODEL_BYTES][..]);
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    for _ in 0..20_000 {
        let mut w = [0u16; IDENTIFY_WORDS];
        for x in w.iter_mut() {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            *x = s as u16;
        }
        let n = names(&w);
        for text in [n.model(), n.serial()] {
            assert!(text.iter().all(|b| (0x20..0x7f).contains(b)), "{text:?}");
            assert!(text.first() != Some(&b' ') && text.last() != Some(&b' '), "{text:?}");
        }
        assert!(n.model.iter().skip(n.model_len as usize).all(|&b| b == 0));
        assert!(n.serial.iter().skip(n.serial_len as usize).all(|&b| b == 0));
    }
}
