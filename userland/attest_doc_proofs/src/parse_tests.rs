// NONOS Operating System (AGPL-3.0-or-later)
//! What the attestation document parser must refuse.

use crate::doc_parse::parse;
use crate::kernel_document::{
    AttestationDoc, DOC_VERSION, IOMMU_AMD_VI, IOMMU_INTEL_VTD, IOMMU_NONE,
};

const CHALLENGE: [u8; 32] = [7u8; 32];

/// Where the completeness byte sits, for the tests that write a byte the kernel
/// never would.
const COMPLETE_AT: usize = 8 + 4 + 32 + 32 + 4;
const VENDOR_AT: usize = COMPLETE_AT + 1;
const ENFORCING_AT: usize = VENDOR_AT + 1;
const ATTEST_LEN_AT: usize = ENFORCING_AT + 1 + 4;

/// A document exactly as the kernel encodes one, from the kernel's own encoder.
fn encoded(
    challenge: &[u8; 32],
    attest: &[u8],
    sig: &[u8],
    count: u32,
    vendor: u8,
    enforcing: bool,
    grants: u32,
) -> Vec<u8> {
    AttestationDoc {
        challenge: *challenge,
        registry_root: [0xAB; 32],
        capsule_count: count,
        registry_complete: true,
        iommu_vendor: vendor,
        iommu_enforcing: enforcing,
        unconfined_grants: grants,
        attest: attest.to_vec(),
        signature: sig.to_vec(),
        ak_public: [0xCD; 64],
    }
    .encode()
}

/// The same, with the completeness byte set to `complete` rather than a bool,
/// so the tests can write the bytes the kernel never does.
fn doc(challenge: &[u8; 32], attest: &[u8], sig: &[u8], complete: u8, count: u32) -> Vec<u8> {
    let mut v = encoded(challenge, attest, sig, count, IOMMU_INTEL_VTD, true, 13);
    v[COMPLETE_AT] = complete;
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
    d[8..12].copy_from_slice(&(DOC_VERSION + 1).to_be_bytes());
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
    let at = ATTEST_LEN_AT;
    for claimed in [u32::MAX, 0x7FFF_FFFF, 5000, d.len() as u32] {
        d[at..at + 4].copy_from_slice(&claimed.to_be_bytes());
        assert!(parse(&d, &CHALLENGE).is_none(), "attest_len {claimed} was accepted");
    }
}

#[test]
fn an_overlong_signature_length_is_refused() {
    let mut d = good();
    let sig_at = ATTEST_LEN_AT + 4 + 4;
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

/// The DMA posture comes through as the kernel wrote it: which IOMMU, whether it
/// enforces, and how many mappings go around it.
#[test]
fn the_dma_posture_is_read_as_written() {
    let d = parse(&good(), &CHALLENGE).unwrap();
    assert_eq!(d.iommu_vendor, IOMMU_INTEL_VTD);
    assert!(d.iommu_enforcing);
    assert_eq!(d.unconfined_grants, 13);
    let none = encoded(&CHALLENGE, &[1], &[9; 8], 3, IOMMU_NONE, false, 0);
    let d = parse(&none, &CHALLENGE).unwrap();
    assert_eq!((d.iommu_vendor, d.iommu_enforcing, d.unconfined_grants), (IOMMU_NONE, false, 0));
}

/// The DMA row names the unit from the kernel's own vendor values, so a
/// machine with no IOMMU is told apart from one whose IOMMU is bypassed.
#[test]
fn each_kernel_vendor_names_its_unit() {
    let cases: [(u8, Option<&[u8]>); 3] =
        [(IOMMU_NONE, None), (IOMMU_INTEL_VTD, Some(b"VT-d")), (IOMMU_AMD_VI, Some(b"AMD-Vi"))];
    for (vendor, unit) in cases {
        let v = encoded(&CHALLENGE, &[1], &[9; 8], 3, vendor, vendor != IOMMU_NONE, 0);
        let d = parse(&v, &CHALLENGE).unwrap();
        assert_eq!(d.iommu_unit(), unit, "vendor {vendor}");
    }
}

/// A vendor the parser does not know is a layout it would be guessing at.
#[test]
fn an_unknown_vendor_is_refused() {
    for vendor in [3u8, 0x7F, 0xFF] {
        let mut d = good();
        d[VENDOR_AT] = vendor;
        assert!(parse(&d, &CHALLENGE).is_none(), "vendor {vendor} was accepted");
    }
}

/// As with completeness, only a 1 says the unit enforces.
#[test]
fn an_enforcing_byte_that_is_not_one_is_not_yes() {
    for byte in [0u8, 2, 0xFF] {
        let mut d = good();
        d[ENFORCING_AT] = byte;
        let d = parse(&d, &CHALLENGE).unwrap();
        assert!(!d.iommu_enforcing, "byte {byte} must not read as enforcing");
    }
}

/// The key is a P-256 point, 64 bytes. A key blob of any other length is a
/// document the parser does not understand.
#[test]
fn a_key_of_any_other_length_is_refused() {
    let d = good();
    let key_len_at = d.len() - 64 - 4;
    assert_eq!(&d[key_len_at..key_len_at + 4], &64u32.to_be_bytes());
    for claimed in [0u32, 63, 65, u32::MAX] {
        let mut v = d.clone();
        v[key_len_at..key_len_at + 4].copy_from_slice(&claimed.to_be_bytes());
        assert!(parse(&v, &CHALLENGE).is_none(), "key length {claimed} was accepted");
    }
}
