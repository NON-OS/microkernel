// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded cases: devices with regions in several spaces, each defined in
//! the DSDT or a later table, its _REG in the same table, a later one or
//! none; then connects (repeats allowed) and the later tables' loads, by
//! three paths, interleaved.

use crate::enc::*;

pub const SPACES: [u8; 4] = [0, 1, 2, 3];

pub struct Dev {
    pub tag: u64,
    pub table: usize,
    pub reg_in: Option<usize>,
    pub spaces: Vec<u8>,
}

pub enum Step {
    Connect(u8),
    /// The table, and the path: directly, by LoadTable, or by Load.
    Load(usize, u8),
}

pub struct Case {
    pub devs: Vec<Dev>,
    pub ssdts: usize,
    pub steps: Vec<Step>,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn case(seed: u64) -> Case {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let ssdts = below(s, 3) as usize;
    let devs = (0..1 + below(s, 5))
        .map(|tag| {
            let table = below(s, ssdts as u64 + 1) as usize;
            let spaces = (0..below(s, 4)).map(|_| SPACES[below(s, 4) as usize]).collect();
            let reg_in =
                (below(s, 5) != 0).then(|| table + below(s, (ssdts - table) as u64 + 1) as usize);
            Dev { tag, table, reg_in, spaces }
        })
        .collect();
    let (mut connects, mut next, mut steps) = (1 + below(s, 5), 1, Vec::new());
    while connects > 0 || next <= ssdts {
        if next <= ssdts && (connects == 0 || below(s, 2) == 0) {
            steps.push(Step::Load(next, below(s, 3) as u8));
            next += 1;
        } else {
            steps.push(Step::Connect(SPACES[below(s, 4) as usize]));
            connects -= 1;
        }
    }
    Case { devs, ssdts, steps }
}

/// What each _REG writes to Debug: Arg0, Arg1 and its device's tag.
pub fn said(tag: u64, space: u8, arg1: u64) -> String {
    format!("Integer({})", space as u64 + (arg1 << 4) + (tag << 8))
}

pub fn reg_body(tag: u64) -> Vec<u8> {
    let shifted = e(0x79, &[&arg(1), &int(4), &[0]]);
    let sum = e(0x72, &[&arg(0), &e(0x72, &[&shifted, &int(tag << 8), &[0]]), &[0]]);
    store(&sum, &op(0x5B31))
}
