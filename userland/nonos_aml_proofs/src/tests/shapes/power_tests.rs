// NONOS Operating System (AGPL-3.0-or-later)
//! _BIF, _BIX and _BST under lying shapes. The model (ACPI 6.5, 10.2.2;
//! recalled, unverified): an integer field is unknown when it is missing,
//! not an integer, wider than 32 bits, or 0xFFFFFFFF; a string field is
//! empty unless a String or Buffer, read to its NUL, 64 bytes at most; a
//! package too short for its fields is refused; and a _BST state with a
//! bit past 3, or charging and discharging at once, is refused whole, so
//! no lying critical bit reaches the power-off.

use super::gen::{lie, Rng};
use super::run::{elements, hold, word};
use crate::object::Object;
use crate::power::{parse_bif, parse_bix, BatteryInfo};

fn field(e: &[Object], i: usize) -> Option<u32> {
    word(e, i).filter(|v| *v != u32::MAX)
}

fn words(e: &[Object], i: usize) -> Vec<u8> {
    match e.get(i) {
        Some(Object::String(s) | Object::Buffer(s)) => {
            let s = s.borrow();
            let end = s.iter().position(|b| *b == 0).unwrap_or(s.len());
            s[..end.min(64)].to_vec()
        }
        _ => Vec::new(),
    }
}

fn info(e: &[Object], at: usize, text: usize, cycle_count: Option<u32>) -> BatteryInfo {
    BatteryInfo {
        milliamp: field(e, at) == Some(1),
        design_capacity: field(e, at + 1),
        last_full: field(e, at + 2),
        rechargeable: field(e, at + 3) == Some(1),
        design_voltage: field(e, at + 4),
        warning: field(e, at + 5),
        low: field(e, at + 6),
        cycle_count,
        model: words(e, text),
        serial: words(e, text + 1),
        kind: words(e, text + 2),
        oem: words(e, text + 3),
    }
}

fn text(r: &mut Rng) -> Object {
    let n = [0, 5, 70][r.below(3) as usize];
    // A long string has no NUL, so only the 64-byte bound ends it.
    let nul = if n > 64 { 2 } else { 3 };
    let s: Vec<u8> = (0..n).map(|_| [b'A', b'7', 0][r.below(nul) as usize]).collect();
    if r.below(2) == 0 {
        Object::string(&s)
    } else {
        Object::buffer(s)
    }
}

fn battery(r: &mut Rng, ints: usize) -> Object {
    let mut e: Vec<Object> = (0..ints).map(|i| r.int([1, 5000, 4800][i % 3])).collect();
    e.extend((0..4).map(|_| text(r)));
    lie(r, e)
}

#[test]
fn bif_and_bix_are_their_models() {
    let bif = |o: &Object| elements(o).filter(|e| e.len() >= 13).map(|e| info(&e, 0, 9, None));
    hold("_BIF", 0xB1F, |r| battery(r, 9), parse_bif, bif);
    let bix =
        |o: &Object| elements(o).filter(|e| e.len() >= 20).map(|e| info(&e, 1, 16, field(&e, 8)));
    hold("_BIX", 0xB18, |r| battery(r, 16), parse_bix, bix);
}

#[path = "bst_tests.rs"]
mod bst;
