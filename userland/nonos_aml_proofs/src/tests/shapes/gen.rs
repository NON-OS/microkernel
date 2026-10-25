// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded objects: an honest package, then one lie told of it: an element
//! of another type or a reference where an integer belongs, too few or too
//! many elements, the package wrapped in another, or no package at all.

use crate::object::Object;

/// Seeds per decoder, and the least honest and refused results a run must
/// show: our choice, from no source.
pub const SEEDS: u64 = 3000;
pub const FLOOR: usize = 300;

pub struct Rng(pub u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }

    pub fn int_below(&mut self, n: u64) -> Object {
        let v = self.below(n);
        self.int(v)
    }

    /// Mostly `honest`; sometimes unknown, past 32 bits, or anything.
    pub fn int(&mut self, honest: u64) -> Object {
        Object::Integer(match self.below(12) {
            0 => 0xFFFF_FFFF,
            1 => 1 << 40 | honest,
            2 => self.next(),
            _ => honest,
        })
    }
}

/// An element of a type no slot wants, or an integer no field can take.
pub fn stranger(r: &mut Rng) -> Object {
    match r.below(7) {
        0 => Object::string(b"12"),
        1 => Object::buffer(vec![1, 2, 3]),
        2 => Object::package(vec![Object::Integer(1)]),
        3 => Object::Named(5, 0),
        4 => Object::Uninit,
        5 => Object::Integer(1 << 40),
        _ => Object::Table(1),
    }
}

/// The honest elements as a package, or with one lie told of them.
pub fn lie(r: &mut Rng, mut e: Vec<Object>) -> Object {
    let at = |r: &mut Rng, n: usize| r.below(n as u64) as usize;
    match r.below(9) {
        0 if !e.is_empty() => {
            let i = at(r, e.len());
            e[i] = stranger(r);
        }
        1 => e.truncate(at(r, e.len())),
        2 => e.push(stranger(r)),
        3 => return Object::package(vec![Object::package(e)]),
        4 => return stranger(r),
        _ => {}
    }
    Object::package(e)
}
