/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Synthetic patch files in the layout `image.rs` reads, from made-up
//! sections and octets; and a model holding one as its package's file.

use super::files::{File, KIND_FIRST};
use super::model::Model;

/// A made-up section: load address and its octets (none for a section
/// that is not downloaded).
pub struct Part {
    pub addr: u32,
    pub data: Vec<u8>,
}

pub fn octets(len: usize, seed: u8) -> Vec<u8> {
    (0..len).map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed)).collect()
}

pub fn image(parts: &[Part]) -> Vec<u8> {
    let mut img = vec![0u8; 96 + 64 * parts.len()];
    img[..16].copy_from_slice(b"20261028000000\0\0");
    img[32 + 12..32 + 16].copy_from_slice(&(parts.len() as u32).to_le_bytes());
    for (i, p) in parts.iter().enumerate() {
        let (map, at) = (96 + 64 * i, img.len() as u32);
        let words = [0x0003_0002, at, p.data.len() as u32, p.addr, p.data.len() as u32];
        for (w, v) in words.iter().enumerate() {
            img[map + 4 * w..map + 4 * w + 4].copy_from_slice(&v.to_le_bytes());
        }
        img.extend_from_slice(&p.data);
    }
    img
}

/// Three sections as a patch has them: two to download, one not.
pub fn three() -> Vec<Part> {
    vec![
        Part { addr: 0x0090_0000, data: octets(700, 1) },
        Part { addr: 0x0091_8000, data: octets(250, 2) },
        Part { addr: 0, data: Vec::new() },
    ]
}

/// The pins for `img` as both chips' file, at the capsule's kinds.
pub fn pins(img: &[u8]) -> [File; 2] {
    let pin = |k| File { kind: KIND_FIRST + k, len: img.len(), sha256: nonos_hash::sha256(img) };
    [pin(0), pin(1)]
}

/// `m` with `img` in the package as both chips' file.
pub fn holding(mut m: Model, img: &[u8]) -> Model {
    m.files.insert(KIND_FIRST, img.to_vec());
    m.files.insert(KIND_FIRST + 1, img.to_vec());
    m
}
