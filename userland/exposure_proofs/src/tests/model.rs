// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's capability bits, read from its source, and seeded masks.

use std::collections::BTreeMap;

/// Name to bit, from `src/capabilities/types/defs.rs`.
pub fn kernel_bits() -> BTreeMap<String, u64> {
    let defs = std::fs::read_to_string("../../src/capabilities/types/defs.rs").unwrap();
    let mut out = BTreeMap::new();
    for l in defs.lines().map(str::trim) {
        let Some((name, rest)) = l.split_once(" = 1 << ") else { continue };
        let shift: u32 = rest.trim_end_matches(',').parse().unwrap();
        out.insert(name.to_string(), 1u64 << shift);
    }
    assert!(out.len() >= 37, "read {} capabilities", out.len());
    out
}

pub fn any(bits: &BTreeMap<String, u64>, mask: u64, names: &[&str]) -> bool {
    names.iter().any(|n| mask & bits[*n] != 0)
}

/// Every single bit, every pair, and seeded masks.
pub fn masks() -> Vec<u64> {
    let mut out: Vec<u64> = (0..40).map(|i| 1u64 << i).collect();
    for i in 0..40 {
        out.extend((i + 1..40).map(|j| (1u64 << i) | (1u64 << j)));
    }
    let mut x = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..4000 {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        out.push(x & ((1 << 40) - 1));
    }
    out
}

/// A registry entry, as the kernel writes it.
pub fn entry(pid: u32, caps: u64, authority: u8) -> Vec<u8> {
    let mut e = vec![0u8; 45];
    e[..4].copy_from_slice(&pid.to_be_bytes());
    e[4..10].copy_from_slice(&[pid as u8, 1, 2, 3, 4, 5]);
    e[36..44].copy_from_slice(&caps.to_be_bytes());
    e[44] = authority;
    e
}
