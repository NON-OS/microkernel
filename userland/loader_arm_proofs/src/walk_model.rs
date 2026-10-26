// NONOS Operating System (AGPL-3.0-or-later)
//! Operands worked out from the Arm ARM's definitions of CLIDR_EL1,
//! CCSIDR_EL1 (both layouts) and DC CISW, independently of the walk.

use crate::walk::{run, Fixture};

/// A level's shape: log2 of its line in bytes, ways, sets.
pub(crate) type Shape = (u64, u64, u64);

fn ccsidr((line, ways, sets): Shape, wide: bool) -> u64 {
    let (way_at, set_at) = if wide { (3, 32) } else { (3, 13) };
    (line - 4) | (ways - 1) << way_at | (sets - 1) << set_at
}

/// CLIDR_EL1 from each level's Ctype and the level of coherency.
fn clidr(ctypes: &[u64], loc: u64) -> u64 {
    ctypes.iter().enumerate().fold(loc << 24, |c, (i, t)| c | t << (3 * i))
}

pub(crate) fn fixture(ctypes: &[u64], loc: u64, shapes: &[Shape], wide: bool) -> Fixture {
    let mut ccsidr_all = [0u64; 7];
    for (slot, &shape) in ccsidr_all.iter_mut().zip(shapes) {
        *slot = ccsidr(shape, wide);
    }
    Fixture { clidr: clidr(ctypes, loc), mmfr2: if wide { 1 << 20 } else { 0 }, ccsidr: ccsidr_all }
}

/// Every operand, ways and sets counted down: way in the top A bits of 32,
/// A = ceil(log2(ways)), set above the line offset, level above bit 0.
pub(crate) fn expected(ctypes: &[u64], loc: u64, shapes: &[Shape]) -> Vec<u64> {
    let mut out = Vec::new();
    for (level, &(line, ways, sets)) in shapes.iter().enumerate().take(loc as usize) {
        if ctypes.get(level).copied().unwrap_or(0) < 2 {
            continue;
        }
        let a = 64 - (ways - 1).leading_zeros() as u64;
        for way in (0..ways).rev() {
            for set in (0..sets).rev() {
                let way_bits = if a == 0 { 0 } else { way << (32 - a) };
                out.push(way_bits | set << line | (level as u64) << 1);
            }
        }
    }
    out
}

pub(crate) fn check(ctypes: &[u64], loc: u64, shapes: &[Shape]) {
    for wide in [false, true] {
        let got = run(&fixture(ctypes, loc, shapes, wide));
        assert_eq!(got, expected(ctypes, loc, shapes), "wide layout: {wide}");
    }
}
