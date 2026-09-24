// NONOS Operating System (AGPL-3.0-or-later)
//! What the attestation document parser must refuse.

use crate::doc_parse::parse;

const CHALLENGE: [u8; 32] = [7u8; 32];

/// Build a document the way the kernel encodes one: magic, version, challenge,
/// registry root, capsule count, the completeness byte, then the two
/// length-prefixed blobs. Big-endian throughout, matching the TPM structures.
fn doc(challenge: &[u8; 32], attest: &[u8], sig: &[u8], complete: u8, count: u32) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(b"NONOSATT");
    v.extend_from_slice(&1u32.to_be_bytes());
    v.extend_from_slice(challenge);
    v.extend_from_slice(&[0xAB; 32]);
    v.extend_from_slice(&count.to_be_bytes());
    v.push(complete);
    v.extend_from_slice(&(attest.len() as u32).to_be_bytes());
    v.extend_from_slice(attest);
    v.extend_from_slice(&(sig.len() as u32).to_be_bytes());
    v.extend_from_slice(sig);
    v
}

fn good() -> Vec<u8> {
    doc(&CHALLENGE, &[1, 2, 3, 4], &[9; 72], 1, 44)
}

#[test]
fn reads_a_well_formed_document() {
    let d = parse(&good(), &CHALLENGE).expect("a well formed document must parse");
    assert_eq!(d.capsule_count, 44);
    assert_eq!(d.registry_root, [0xAB; 32]);
    assert!(d.registry_complete);
    assert!(d.challenge_echoed);
    assert_eq!(d.attest_len, 4);
    assert_eq!(d.signature_len, 72);
}

/// The anti-replay check. A document carrying someone else's challenge parses,
/// because it is structurally valid, but must not claim the challenge matched:
/// that flag is the only part of the document this capsule verifies itself.
#[test]
fn a_replayed_challenge_does_not_echo() {
    let stale = doc(&[0u8; 32], &[1, 2, 3, 4], &[9; 72], 1, 44);
    let d = parse(&stale, &CHALLENGE).expect("structurally valid");
    assert!(!d.challenge_echoed, "a document answering another challenge must not read as fresh");
}

#[test]
fn an_incomplete_registry_is_carried_through() {
    let d = parse(&doc(&CHALLENGE, &[1], &[9; 8], 0, 3), &CHALLENGE).unwrap();
    assert!(!d.registry_complete);
}

/// Any byte other than 1 means the machine did not say yes.
#[test]
fn a_completeness_byte_that_is_not_one_is_not_yes() {
    for byte in [0u8, 2, 0xFF] {
        let d = parse(&doc(&CHALLENGE, &[1], &[9; 8], byte, 3), &CHALLENGE).unwrap();
        assert!(!d.registry_complete, "byte {byte} must not read as complete");
    }
}

#[test]
fn wrong_magic_is_refused() {
    let mut d = good();
    d[0] = b'X';
    assert!(parse(&d, &CHALLENGE).is_none());
}

/// A verifier that parses a version it does not know is guessing at a layout.
#[test]
fn an_unknown_version_is_refused() {
    let mut d = good();
    d[8..12].copy_from_slice(&2u32.to_be_bytes());
    assert!(parse(&d, &CHALLENGE).is_none());
}

/// Truncation at every length, not just a convenient one: each prefix must be
/// refused rather than read as a shorter document that happens to parse.
#[test]
fn every_truncation_is_refused() {
    let d = good();
    for cut in 0..d.len() {
        assert!(parse(&d[..cut], &CHALLENGE).is_none(), "a document cut to {cut} bytes parsed");
    }
    assert!(parse(&d, &CHALLENGE).is_some(), "only the whole document parses");
}

/// Trailing bytes are as wrong as missing ones. A document that ends past where
/// its own lengths say it ends is one an attacker has appended to.
#[test]
fn trailing_bytes_are_refused() {
    let mut d = good();
    d.push(0);
    assert!(parse(&d, &CHALLENGE).is_none());
}

/// The length field is attacker-shaped in the case that matters, so it must not
/// be used to index before it is checked against what is actually there.
#[test]
fn an_overlong_attest_length_is_refused_and_does_not_panic() {
    let mut d = good();
    let at = 8 + 4 + 32 + 32 + 4 + 1;
    for claimed in [u32::MAX, 0x7FFF_FFFF, 5000, d.len() as u32] {
        d[at..at + 4].copy_from_slice(&claimed.to_be_bytes());
        assert!(parse(&d, &CHALLENGE).is_none(), "attest_len {claimed} was accepted");
    }
}

#[test]
fn an_overlong_signature_length_is_refused() {
    let mut d = good();
    let sig_at = 8 + 4 + 32 + 32 + 4 + 1 + 4 + 4;
    d[sig_at..sig_at + 4].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(parse(&d, &CHALLENGE).is_none());
}

/// An empty signature is structurally consistent and still worthless. The parser
/// reports it faithfully rather than rejecting it, because deciding that a
/// zero-length signature is unacceptable is the caller's judgement, not the
/// wire format's.
#[test]
fn an_empty_signature_parses_and_is_reported_as_empty() {
    let d = parse(&doc(&CHALLENGE, &[1], &[], 1, 1), &CHALLENGE).unwrap();
    assert_eq!(d.signature_len, 0);
}

#[test]
fn arbitrary_noise_is_refused() {
    for len in [0usize, 1, 8, 12, 44, 81, 200] {
        let noise: Vec<u8> = (0..len).map(|i| (i * 37 % 251) as u8).collect();
        assert!(parse(&noise, &CHALLENGE).is_none(), "{len} bytes of noise parsed");
    }
}
