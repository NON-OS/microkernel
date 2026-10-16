// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded hostile firmware for _OSC: an answer of any shape, word count,
//! status bits and capability bits, most of them granting more than asked.

use crate::exec::{Fault, FaultKind};
use crate::object::Object;

pub fn roll(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n
}

/// Status bits, rarely set: query, failure, unknown UUID, unknown
/// revision, masked (ACPI 6.5, 6.2.11; recalled, unverified).
fn status(s: &mut u64) -> u32 {
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 4, 8, 16, 0x1E][roll(s, 17) as usize]
}

/// Firmware's answer to `n` words sent.
pub fn answer(s: &mut u64, n: usize) -> Result<Object, Fault> {
    let words = match roll(s, 24) {
        0 => n.saturating_sub(1),
        1 => n + 2,
        _ => n,
    };
    let mut w: Vec<u32> =
        (0..words).map(|_| (roll(s, 1 << 32) as u32) | roll(s, 2) as u32).collect();
    if let Some(first) = w.first_mut() {
        *first = status(s);
    }
    let mut bytes: Vec<u8> = w.iter().flat_map(|v| v.to_le_bytes()).collect();
    match roll(s, 30) {
        0 => Ok(Object::Integer(7)),
        1 => Ok(Object::package(vec![Object::buffer(bytes)])),
        2 => Err(Fault { kind: FaultKind::Type, table: 0, at: 0 }),
        3 => {
            bytes.truncate(bytes.len().saturating_sub(1));
            Ok(Object::buffer(bytes))
        }
        _ => Ok(Object::buffer(bytes)),
    }
}

/// What a caller asks and may claim, as the capsule's callers do: a status
/// word, then capability words.
pub fn ask(s: &mut u64) -> (Vec<u32>, Vec<u32>) {
    let n = 1 + roll(s, 3) as usize;
    let mut asked: Vec<u32> = (0..n).map(|_| roll(s, 1 << 32) as u32 & 0xFF).collect();
    let mut claim: Vec<u32> = (0..n).map(|_| [0, u32::MAX, 0x0F][roll(s, 3) as usize]).collect();
    asked.insert(0, 0);
    claim.insert(0, 0);
    (asked, claim)
}
