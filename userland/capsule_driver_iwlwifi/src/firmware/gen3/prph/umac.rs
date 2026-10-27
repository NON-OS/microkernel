// NONOS Operating System (AGPL-3.0-or-later)

//! UMAC peripheral registers, which sit `UMAC_PRPH_OFFSET` above their base,
//! read and written with the MAC held awake in the family's CSR layout.

use super::super::csr::CsrLayout;
use super::super::region::Clock;
use super::{grab, read_no_grab, release, write_no_grab, UMAC_PRPH_OFFSET};
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
    write_no_grab(m, addr.wrapping_add(UMAC_PRPH_OFFSET), val);
    release(m, csr);
    true
}

/// Read a UMAC register, or `None` if the MAC did not wake.
pub fn read_umac<M: Mmio, C: Clock>(m: &M, c: &mut C, csr: CsrLayout, addr: u32) -> Option<u32> {
    if !grab(m, c, csr) {
        return None;
    }
    let v = read_no_grab(m, addr.wrapping_add(UMAC_PRPH_OFFSET));
    release(m, csr);
    Some(v)
}
