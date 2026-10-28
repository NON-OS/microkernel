// NONOS Operating System (AGPL-3.0-or-later)

//! Synthetic firmware images in the real layouts (`mt76_connac_mcu.h`), so
//! the download is proven with no MediaTek file: a ROM patch of a 96-byte
//! big-endian header and 64-byte section records, and RAM code whose
//! regions run from the start, then 40-byte region records, then the
//! 36-byte trailer. The bytes are patterns, never firmware.

/// A patch image: one section per `(addr, bytes, sec_key_idx)`.
pub fn patch(sections: &[(u32, Vec<u8>, u32)]) -> Vec<u8> {
    let table = 96 + 64 * sections.len();
    let mut img = vec![0u8; table];
    img[..16].copy_from_slice(b"20260724143257a\n");
    img[16..20].copy_from_slice(b"ALPS");
    img[44..48].copy_from_slice(&(sections.len() as u32).to_be_bytes());
    for (i, (addr, bytes, key)) in sections.iter().enumerate() {
        let at = 96 + 64 * i;
        let words =
            [0x40002, img.len() as u32, bytes.len() as u32, *addr, bytes.len() as u32, *key];
        for (j, w) in words.iter().enumerate() {
            img[at + 4 * j..at + 4 * j + 4].copy_from_slice(&w.to_be_bytes());
        }
        img.extend_from_slice(bytes);
    }
    img
}

/// A RAM image: one region per `(addr, bytes, feature_set)`.
pub fn ram(regions: &[(u32, Vec<u8>, u8)]) -> Vec<u8> {
    let mut img: Vec<u8> = regions.iter().flat_map(|r| r.1.clone()).collect();
    img.extend_from_slice(&[0u8; 16]); // what the shipped files leave between
    for (addr, bytes, feature) in regions {
        let mut r = [0u8; 40];
        r[16..20].copy_from_slice(&addr.to_le_bytes());
        r[20..24].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
        r[24] = *feature;
        img.extend_from_slice(&r);
    }
    let mut t = [0u8; 36];
    (t[0], t[2], t[3]) = (0x10, regions.len() as u8, 2);
    img.extend_from_slice(&t);
    img
}

/// A pattern of `n` bytes, distinct per `seed`.
pub fn pattern(seed: u8, n: usize) -> Vec<u8> {
    (0..n).map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed)).collect()
}

/// An MT7922-shaped patch: one AES-keyed section at 0x900000.
pub fn patch_7922() -> Vec<u8> {
    patch(&[(0x900000, pattern(1, 9000), 0x0100_0000)])
}

/// MT7922-shaped RAM code: the start region (encrypted, override), two
/// more, and the CLC tables (not downloaded).
pub fn ram_7922() -> Vec<u8> {
    ram(&[
        (0x91b000, pattern(2, 5000), 0x21),
        (0x2015c00, pattern(3, 4096), 0x01),
        (0x404400, pattern(4, 100), 0x01),
        (0, pattern(5, 700), 0x40),
    ])
}
