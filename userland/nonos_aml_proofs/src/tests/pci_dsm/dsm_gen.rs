// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded firmware `_DSM`s: the UUID it serves (the PCI one, one byte off,
//! or a byte short), and for each revision 0 to 4 what function 0 answers:
//! nothing (the catch-all Buffer() {0}), a buffer of 0 to 10 bytes, an
//! integer, a string or a package. Every other function answers 0x100 plus
//! its index whether or not the mask offers it, and every call writes
//! Arg1 << 8 | Arg2 to Debug first, so a call the mask forbids shows.

use crate::pci_dsm::PCI_DSM_UUID;

#[derive(Clone, Debug)]
pub enum Form {
    Absent,
    Buf(Vec<u8>),
    Int(u64),
    Str,
    Pkg,
}

pub struct Fw {
    pub uuid: Vec<u8>,
    pub forms: Vec<Form>,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn fw(seed: u64) -> Fw {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut uuid = PCI_DSM_UUID.to_vec();
    match below(s, 5) {
        0 => uuid[below(s, 16) as usize] ^= 1 << below(s, 8),
        1 => uuid.truncate(15),
        _ => {}
    }
    let forms = (0..5)
        .map(|_| match below(s, 6) {
            0 => Form::Absent,
            1 => Form::Int(below(s, u64::MAX) | below(s, 2)),
            2 => Form::Str,
            3 => Form::Pkg,
            _ => {
                let mut b: Vec<u8> = (0..below(s, 11)).map(|_| below(s, 256) as u8).collect();
                if let (Some(x), 0) = (b.first_mut(), below(s, 3)) {
                    *x |= 1;
                }
                Form::Buf(b)
            }
        })
        .collect();
    Fw { uuid, forms }
}
