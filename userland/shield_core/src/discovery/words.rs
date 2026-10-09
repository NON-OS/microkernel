//! The four words a commitment occupies, in both directions.
//! One conversion site, so a byte order cannot end up reversed at a single call.

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

/// The four words the pool holds, as field elements.
pub fn quad(words: &[u64; 4]) -> [Fp; RATE] {
    let mut out = [Fp::ZERO; RATE];
    for (slot, word) in out.iter_mut().zip(words.iter()) {
        *slot = Fp::from_u64(*word);
    }
    out
}

/// Field elements back to the four words a record stores.
pub fn words(elems: [Fp; RATE]) -> [u64; 4] {
    let mut out = [0u64; 4];
    for (slot, e) in out.iter_mut().zip(elems.iter()) {
        *slot = e.value();
    }
    out
}
