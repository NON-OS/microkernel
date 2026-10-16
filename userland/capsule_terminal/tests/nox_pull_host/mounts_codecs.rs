// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! The inflate and base64 crates, mounted by path.

#[path = "../../../inflate/src"]
pub mod inflate {
    // Every module of the crate, as its lib.rs declares them: each one
    // reaches its siblings through `super::`, so a partial list does not build.
    #[path = "adler32.rs"]
    pub mod adler32;
    #[path = "align.rs"]
    pub mod align;
    #[path = "bits.rs"]
    pub mod bits;
    #[path = "codes.rs"]
    pub mod codes;
    #[path = "copy.rs"]
    pub mod copy;
    #[path = "copy_words.rs"]
    pub mod copy_words;
    #[path = "crc32.rs"]
    pub mod crc32;
    #[path = "dynamic.rs"]
    pub mod dynamic;
    #[path = "emit.rs"]
    pub mod emit;
    #[path = "fast.rs"]
    pub mod fast;
    #[path = "fixed.rs"]
    pub mod fixed;
    #[path = "gzip.rs"]
    pub mod gzip;
    #[path = "gzip_header.rs"]
    pub mod gzip_header;
    #[path = "gzip_member.rs"]
    pub mod gzip_member;
    #[path = "huff.rs"]
    pub mod huff;
    #[path = "huff_build.rs"]
    pub mod huff_build;
    #[path = "huff_fill.rs"]
    pub mod huff_fill;
    #[path = "huff_sub.rs"]
    pub mod huff_sub;
    #[path = "inflate_raw.rs"]
    pub mod inflate_raw;
    #[path = "members.rs"]
    pub mod members;
    #[path = "meta.rs"]
    pub mod meta;
    #[path = "out.rs"]
    pub mod out;
    #[path = "stored.rs"]
    pub mod stored;
    #[path = "tables.rs"]
    pub mod tables;
    #[path = "types.rs"]
    pub mod types;
    #[path = "zlib.rs"]
    pub mod zlib;
}

#[path = "../../../base64/src"]
pub mod base64 {
    #[path = "decode.rs"]
    pub mod decode;
    #[path = "encode.rs"]
    pub mod encode;
}
