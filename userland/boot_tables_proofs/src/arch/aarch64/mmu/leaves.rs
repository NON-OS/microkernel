// NONOS Operating System (AGPL-3.0-or-later)
//! Every leaf the boot tables hold, found by visiting each valid entry from
//! level 0 down rather than by probing chosen addresses: a property checked
//! here holds for the whole of both halves, not for the addresses a test
//! thought to ask about.

use super::attributes::{PTE_ADDR_MASK, PTE_TABLE, PTE_VALID};
pub use super::attributes::{PTE_GP as GP, PTE_PXN as PXN, PTE_UXN as UXN};

/// AP[1]: EL0 may reach the mapping. AP[2]: it is read only.
pub const AP_EL0: u64 = super::attributes::PTE_AP_RW_ALL;
pub const AP_READ_ONLY: u64 = 1 << 7;
use super::state;
use super::table::PageTable;

/// A leaf, the virtual address it starts at and the level it sits at.
#[derive(Debug, Clone, Copy)]
pub struct Mapped {
    pub va: u64,
    pub level: usize,
    pub descriptor: u64,
}

/// Levels 0 to 3 of a 4 KiB granule index bits 47:39, 38:30, 29:21, 20:12.
const SHIFT: [u32; 4] = [39, 30, 21, 12];

pub fn leaves() -> Vec<Mapped> {
    let mut out = Vec::new();
    // SAFETY: the tables are only read, after `probe::build` wrote them.
    visit(unsafe { &*state::l0() }, 0, 0, &mut out);
    out
}

fn visit(table: &PageTable, level: usize, base: u64, out: &mut Vec<Mapped>) {
    for index in 0..512usize {
        let entry = table.entry(index);
        if entry & PTE_VALID == 0 {
            continue;
        }
        let va = canonical(base | (index as u64) << SHIFT[level]);
        // A level 3 entry with bit 1 set is a page; above it, a table.
        if level < 3 && entry & PTE_TABLE != 0 {
            // SAFETY: a table address the builder wrote, a host pointer here.
            let next = unsafe { &*((entry & PTE_ADDR_MASK) as *const PageTable) };
            visit(next, level + 1, va, out);
        } else {
            assert!(level != 0, "a level 0 entry is never a block on a 4 KiB granule");
            out.push(Mapped { va, level, descriptor: entry });
        }
    }
}

/// Bit 47 set is the TTBR1 half, whose addresses have bits 63:48 set too.
fn canonical(va: u64) -> u64 {
    if va & (1 << 47) != 0 {
        va | 0xFFFF_0000_0000_0000
    } else {
        va
    }
}
