// NONOS Operating System (AGPL-3.0-or-later)
//! A template of up to four descriptors and an end tag, then one lie.

use super::super::gen::Rng;
use super::{descriptor, Truth};

/// The template and, when the lie leaves it known, the truth.
/// No end tag, or bytes after it, leaves the truth as it was.
pub fn template(r: &mut Rng) -> (Vec<u8>, Option<Truth>) {
    let (mut t, mut truth, mut ext, mut starts) = (vec![], Truth::default(), vec![], vec![]);
    for _ in 0..r.below(5) {
        starts.push(t.len());
        descriptor(r, &mut t, &mut truth, &mut ext);
    }
    t.extend([0x79, 0]);
    let honest = match r.below(8) {
        0 if !starts.is_empty() => {
            let at = starts[r.below(starts.len() as u64) as usize];
            match t[at] & 0x80 {
                0 => t[at] = t[at] & !7 | r.below(8) as u8,
                _ => t[at + 1] = r.below(40) as u8,
            }
            false
        }
        1 => {
            t.truncate(r.below(t.len() as u64) as usize);
            false
        }
        2 | 3 => {
            t.truncate(t.len() - 2);
            if r.below(2) == 0 {
                t.extend([0x79, 0, 0x22, 0xFF, 0xFF, 0x89, 6, 0, 0, 1, 9, 9, 9, 9]);
            }
            true
        }
        4 if !ext.is_empty() => {
            t[ext[0] + 4] += 1 + r.below(4) as u8;
            false
        }
        _ => true,
    };
    (t, honest.then_some(truth))
}
