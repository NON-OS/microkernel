// NONOS Operating System (AGPL-3.0-or-later)
//! The same seeded units as an IndexField behind an index/data pair in
//! SystemIO, and as a BankField in a banked window with its one-byte
//! select register above it.

use super::gen::Case;
use crate::enc::*;

pub const IDX_AT: u64 = 0x70;
pub const DAT_AT: u64 = 0x78;

/// AccessType for a register of `n` bytes.
fn acc(n: u64) -> u8 {
    [0, 1, 2, 0, 3, 0, 0, 0, 4][n as usize]
}

fn units(c: &Case) -> Vec<u8> {
    let skip_off = if c.off > 0 { skip(c.off as usize) } else { Vec::new() };
    cat(&[&[c.access | c.rule << 5], &skip_off, &unit("FLD", c.bits as usize)])
}

fn methods() -> Vec<u8> {
    cat(&[
        &method("\\RD", 0, &ret(&name("FLD"))),
        &method("\\WR", 1, &store(&arg(0), &name("FLD"))),
    ])
}

/// `il`: the index register's bytes; `w`: the data register's.
pub fn index_source(c: &Case, il: u64, w: u64) -> Vec<u8> {
    cat(&[
        &region("IXR", 1, IDX_AT, il),
        &field("IXR", acc(il), &[unit("IDX", 8 * il as usize)]),
        &region("DTR", 1, DAT_AT, 8),
        &field("DTR", acc(w), &[unit("DAT", 8 * w as usize)]),
        &pkg(0x5B86, &cat(&[&name("IDX"), &name("DAT"), &units(c)])),
        &methods(),
    ])
}

pub fn select_at(c: &Case) -> u64 {
    c.base + c.len + 0x40
}

pub fn bank_source(c: &Case, bank: u8) -> Vec<u8> {
    cat(&[
        &region("SLR", c.space, select_at(c), 1),
        &field("SLR", 1, &[unit("BNK", 8)]),
        &region("RG", c.space, c.base, c.len),
        &pkg(0x5B87, &cat(&[&name("RG"), &name("BNK"), &int(u64::from(bank)), &units(c)])),
        &methods(),
    ])
}
