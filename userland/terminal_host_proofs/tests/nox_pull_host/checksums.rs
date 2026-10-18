// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Checksum sidecar paths and digest parsing.

use crate::verify;

#[test]
fn verify_sidecar_path_appends_suffix() {
    assert_eq!(verify::sidecar_path(b"/dir/f.txt"), b"/dir/f.txt.sha256".to_vec());
}

#[test]
fn verify_parses_lowercase_digest() {
    let hex = b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let d = verify::parse_sha256_hex(hex).expect("parsed");
    assert_eq!(d[0], 0xe3);
    assert_eq!(d[31], 0x55);
}

#[test]
fn verify_parses_sha256sum_output_with_filename() {
    let line = b"  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  file.bin\n";
    let d = verify::parse_sha256_hex(line).expect("parsed");
    assert_eq!(d[0], 0xe3);
}

#[test]
fn verify_accepts_uppercase_hex() {
    let lo = verify::parse_sha256_hex(
        b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );
    let hi = verify::parse_sha256_hex(
        b"E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
    );
    assert_eq!(lo, hi);
}

#[test]
fn verify_rejects_short_digest() {
    assert!(verify::parse_sha256_hex(b"deadbeef").is_none());
}

#[test]
fn verify_rejects_non_hex_char() {
    let bad = b"g3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    assert!(verify::parse_sha256_hex(bad).is_none());
}
