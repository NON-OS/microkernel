// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded scenarios: a Lock field reached directly, through a BankField or
//! through an IndexField; a fault in the select or index write (its
//! register in a space with no handler), in the data access (past its
//! region), or none; \_GL acquired around it or not; one to three
//! accesses; and firmware free, holding the lock a few looks, holding it
//! for good, or waiting for it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    None,
    Select,
    Data,
}

#[derive(Clone, Copy, Debug)]
pub struct Scene {
    pub kind: u8,
    pub fault: Fault,
    pub outer: bool,
    pub reps: u8,
    /// None: no FACS; else firmware's looks held (u32::MAX for good).
    pub holds: Option<u32>,
    pub waits: bool,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn scene(seed: u64) -> Scene {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let kind = below(s, 3) as u8;
    let fault = [Fault::None, Fault::None, Fault::Select, Fault::Data][below(s, 4) as usize];
    let fault = if kind == 0 && fault == Fault::Select { Fault::None } else { fault };
    let (outer, reps) = (below(s, 2) == 0, 1 + below(s, 3) as u8);
    let holds = [None, Some(0), Some(0), Some(1 + below(s, 3) as u32), Some(u32::MAX)]
        [below(s, 5) as usize];
    Scene { kind, fault, outer, reps, holds, waits: below(s, 3) == 0 }
}
