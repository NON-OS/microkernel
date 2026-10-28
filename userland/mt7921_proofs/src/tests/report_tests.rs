// NONOS Operating System (AGPL-3.0-or-later)

//! Tokens and the reports that free them: 64 tokens, lowest first; a token
//! the chip names that was never handed out ignored; one never reported
//! taken back after `STALE_MS`. The transmit-free report read as
//! `mt7921_mac_tx_free` reads it, pair words not counted, never past its
//! buffer.

use crate::mac::tokens::{Tokens, STALE_MS};
use crate::mac::txfree::tokens;

fn report(words: &[u32], count: u32) -> Vec<u8> {
    let mut b = (12 | 6 << 27 | count << 16).to_le_bytes().to_vec();
    b.extend_from_slice(&[0; 4]);
    b.extend(words.iter().flat_map(|w| w.to_le_bytes()));
    b
}

#[test]
fn sixty_four_tokens_lowest_first_and_only_real_ones_freed() {
    let mut t = Tokens::default();
    for i in 0..64 {
        assert_eq!(t.take(0), Some(i));
    }
    assert_eq!(t.take(0), None);
    assert!(t.free(9) && !t.free(9), "freed once");
    assert!(!t.free(64) && !t.free(u16::MAX), "out of range");
    assert_eq!(t.take(0), Some(9));
    t.clear();
    assert_eq!(t.held(), 0);
}

#[test]
fn a_token_never_reported_is_taken_back_after_the_stale_time() {
    let mut t = Tokens::default();
    t.take(100);
    t.take(100 + STALE_MS);
    t.age(100 + STALE_MS - 1);
    assert_eq!(t.held(), 2);
    t.age(100 + STALE_MS);
    assert_eq!(t.held(), 1, "the first went stale, the second not yet");
}

#[test]
fn a_report_frees_its_tokens_and_skips_pair_words() {
    let mut got = Vec::new();
    let b = report(&[3 << 16, 1 << 31 | 1 << 14, 7 << 16 | 0x7fff << 16], 2);
    tokens(&b, |t| got.push(t));
    assert_eq!(got, [3, 0x7fff], "the pair word extends the count by one");
}

#[test]
fn a_report_claiming_more_than_its_buffer_reads_no_further() {
    let mut got = Vec::new();
    tokens(&report(&[5 << 16], 0x3ff), |t| got.push(t));
    assert_eq!(got, [5]);
    tokens(&[1, 2], |t| got.push(t));
    assert_eq!(got, [5], "shorter than its first word");
}
