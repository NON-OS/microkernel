// NONOS Operating System (AGPL-3.0-or-later)
//! What `ask(f)` should do with a seeded firmware, stated apart from the
//! code: function 0 at revision 3, then 2, until one answers a mask with
//! bit 0 set (a buffer's first eight bytes little-endian, or an integer);
//! then function `f` at that revision only when `f` is 5, 7 or 9 and its
//! bit is set. Returns the answer and every (revision, function) called.

use super::dsm_gen::{Form, Fw};
use crate::pci_dsm::PCI_DSM_UUID;

pub fn mask_of(form: &Form) -> u64 {
    let raw = match form {
        Form::Buf(b) => b.iter().take(8).enumerate().map(|(i, x)| u64::from(*x) << (8 * i)).sum(),
        Form::Int(n) => *n,
        _ => 0,
    };
    if raw % 2 == 1 {
        raw
    } else {
        0
    }
}

pub fn ask(fw: &Fw, f: u64) -> (Option<u64>, Vec<(u64, u64)>) {
    let mut calls = Vec::new();
    if ![5, 7, 9].contains(&f) {
        return (None, calls);
    }
    let served = fw.uuid == PCI_DSM_UUID;
    let mut found = None;
    for rev in [3u64, 2] {
        calls.push((rev, 0));
        let mask = if served { mask_of(&fw.forms[rev as usize]) } else { 0 };
        if mask != 0 {
            found = Some((rev, mask));
            break;
        }
    }
    let Some((rev, mask)) = found else { return (None, calls) };
    if mask & (1 << f) == 0 {
        return (None, calls);
    }
    calls.push((rev, f));
    (Some(0x100 + f), calls)
}
