// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Gzip decoding and base64 encoding used by pulls.

use crate::base64;
use crate::inflate;

#[test]
fn gunzip_hello_roundtrips() {
    let gz: [u8; 25] = [
        0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9,
        0x07, 0x00, 0x86, 0xa6, 0x10, 0x36, 0x05, 0x00, 0x00, 0x00,
    ];
    let out = inflate::gzip::gunzip(&gz).expect("gunzip");
    assert_eq!(&out, b"hello");
}

#[test]
fn base64_encode_basic_auth_credential() {
    assert_eq!(&base64::encode::encode(b"user:pass"), b"dXNlcjpwYXNz");
}
