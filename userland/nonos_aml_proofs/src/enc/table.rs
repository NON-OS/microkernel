// NONOS Operating System (AGPL-3.0-or-later)
//! A definition block: the header of section 5.2.6 with a correct checksum.

pub fn table(sig: &[u8; 4], revision: u8, aml: &[u8]) -> Vec<u8> {
    let mut t = Vec::with_capacity(36 + aml.len());
    t.extend_from_slice(sig);
    t.extend_from_slice(&((36 + aml.len()) as u32).to_le_bytes());
    t.push(revision);
    t.push(0);
    t.extend_from_slice(b"NONOS ");
    t.extend_from_slice(b"FIXTURES");
    t.extend_from_slice(&1u32.to_le_bytes());
    t.extend_from_slice(b"NONO");
    t.extend_from_slice(&1u32.to_le_bytes());
    t.extend_from_slice(aml);
    let sum = t.iter().fold(0u8, |s, b| s.wrapping_add(*b));
    t[9] = 0u8.wrapping_sub(sum);
    t
}
