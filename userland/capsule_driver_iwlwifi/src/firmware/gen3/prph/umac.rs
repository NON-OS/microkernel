// NONOS Operating System (AGPL-3.0-or-later)

//! UMAC peripheral registers, which sit the family's `umac_offset` above
//! their base, in its window (`prph_mask`), read and written with the MAC
//! held awake in the family's CSR layout.

use super::super::csr::CsrLayout;
use super::super::region::Clock;
use super::{grab, read_no_grab, release, write_no_grab};
use crate::regs::Mmio;

/// Write a UMAC register, holding the MAC awake for it.
pub fn write_umac<M: Mmio, C: Clock>(
    m: &M,
    c: &mut C,
    csr: CsrLayout,
    addr: u32,
    val: u32,
) -> bool {
    if !grab(m, c, csr) {
        return false;
    }
    write_no_grab(m, at(csr, addr), val);
    release(m, csr);
    true
}

/// Read a UMAC register, or `None` if the MAC did not wake.
pub fn read_umac<M: Mmio, C: Clock>(m: &M, c: &mut C, csr: CsrLayout, addr: u32) -> Option<u32> {
    if !grab(m, c, csr) {
        return None;
    }
    let v = read_no_grab(m, at(csr, addr));
    release(m, csr);
    Some(v)
}

// The address the window is given for UMAC register `addr`.
fn at(csr: CsrLayout, addr: u32) -> u32 {
    addr.wrapping_add(csr.umac_offset()) & csr.prph_mask()
}
