// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded uses of a loaded table's objects before it is unloaded: take
//! its mutex, signal its event, notify its device, write its field, keep
//! a reference to the field; and whether the method loads it again after.

#[derive(Clone, Copy, Debug)]
pub enum Use {
    Acquire,
    Signal,
    Notify,
    Write,
    Keep,
}

#[derive(Debug)]
pub struct Scene {
    pub uses: Vec<Use>,
    pub reload: bool,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn scene(seed: u64) -> Scene {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let all = [Use::Acquire, Use::Signal, Use::Notify, Use::Write, Use::Keep];
    let uses = (0..1 + below(s, 5)).map(|_| all[below(s, 5) as usize]).collect();
    Scene { uses, reload: below(s, 2) == 0 }
}
