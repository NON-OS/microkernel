// NONOS Operating System (AGPL-3.0-or-later)
//! Holding a decoder to its model over every seed, and showing the run
//! was not vacuous: enough results kept and enough refused.

use super::gen::{Rng, FLOOR, SEEDS};
use crate::object::Object;
use core::fmt::Debug;

/// `make` builds a seeded object; the crate's answer must be the model's.
/// A None answer is a refusal or every field at its safe default.
pub fn hold<T: PartialEq + Debug>(
    what: &str,
    salt: u64,
    mut make: impl FnMut(&mut Rng) -> Object,
    got: impl Fn(&Object) -> Option<T>,
    want: impl Fn(&Object) -> Option<T>,
) {
    let (mut kept, mut refused) = (0, 0);
    for seed in 0..SEEDS {
        let o = make(&mut Rng::new(seed ^ salt));
        let w = want(&o);
        assert_eq!(got(&o), w, "{what}, seed {seed}: {o:?}");
        match w {
            Some(_) => kept += 1,
            None => refused += 1,
        }
    }
    assert!(kept >= FLOOR && refused >= FLOOR, "{what}: {kept} kept, {refused} refused");
}

/// A list the decoder returns, as None when it is empty.
pub fn some<T>(v: Vec<T>) -> Option<Vec<T>> {
    (!v.is_empty()).then_some(v)
}

/// The model's integer slot: an Integer, and nothing else.
pub fn int(e: &[Object], i: usize) -> Option<u64> {
    match e.get(i)? {
        Object::Integer(v) => Some(*v),
        _ => None,
    }
}

/// The same, held to 32 bits.
pub fn word(e: &[Object], i: usize) -> Option<u32> {
    int(e, i).and_then(|v| u32::try_from(v).ok())
}

/// A package's elements, cloned out.
pub fn elements(o: &Object) -> Option<Vec<Object>> {
    match o {
        Object::Package(p) => Some(p.borrow().clone()),
        _ => None,
    }
}
