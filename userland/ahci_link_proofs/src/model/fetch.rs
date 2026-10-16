// NONOS Operating System (AGPL-3.0-or-later)
//! The HBA's reads of what the driver built in memory: slot 0's command
//! header, the command FIS and the PRD entry it points at, and its write of
//! the bytes moved back into the header (PRDBC), each at an address as wide
//! as the HBA's (`reach`).

use core::ptr::{read_volatile, write_volatile};

use super::iommu::{host, host_of};
use super::script::Cmd;
use crate::engine::cmd_header::CmdHeader;
use crate::engine::cmd_table::PRDT_OFFSET;
use crate::engine::fis::FisH2D;
use crate::engine::prdt::PrdtEntry;

/// Where slot 0's PRD sends the data, in host memory, and how many bytes.
pub struct Slot {
    pub data: u64,
    pub bytes: u64,
}

/// `at` as an HBA reads it: whole on one with CAP.S64A (`wide`), else its
/// low 32 bits, since such an HBA treats PxCLBU, PxFBU, CTBAU and DBAU as
/// zero (AHCI 1.3.1, 3.1.1).
pub fn reach(at: u64, wide: bool) -> u64 {
    if wide {
        at
    } else {
        at & 0xffff_ffff
    }
}

fn header(clb: u64, wide: bool) -> CmdHeader {
    // SAFETY: `host_of` found the header inside a live DMA grant.
    unsafe { read_volatile(host_of::<CmdHeader>(reach(clb, wide)) as *const CmdHeader) }
}

fn table(h: &CmdHeader, wide: bool) -> u64 {
    reach(u64::from(h.ctba_low) | u64::from(h.ctba_high) << 32, wide)
}

pub fn command(clb: u64, wide: bool) -> Cmd {
    let at = host_of::<FisH2D>(table(&header(clb, wide), wide));
    // SAFETY: `host_of` found the command FIS inside a live DMA grant.
    let f = unsafe { read_volatile(at as *const FisH2D) };
    let lba = [f.lba5, f.lba4, f.lba3, f.lba2, f.lba1, f.lba0];
    let lba = lba.iter().fold(0u64, |a, &b| a << 8 | u64::from(b));
    Cmd { op: f.command, lba, count: u32::from(f.countl) | u32::from(f.counth) << 8 }
}

/// Slot 0's PRD, its data buffer as a host address the drive may copy
/// through.
pub fn slot(clb: u64, wide: bool) -> Slot {
    let h = header(clb, wide);
    if h.prdtl == 0 {
        return Slot { data: 0, bytes: 0 };
    }
    let at = host_of::<PrdtEntry>(table(&h, wide) + PRDT_OFFSET as u64);
    // SAFETY: `host_of` found the entry inside a live DMA grant.
    let e = unsafe { read_volatile(at as *const PrdtEntry) };
    let data = reach(u64::from(e.dba_low) | u64::from(e.dba_high) << 32, wide);
    let bytes = u64::from(e.dbc & 0x3f_ffff) + 1;
    Slot { data: host(data, bytes), bytes }
}

pub fn set_prdbc(clb: u64, wide: bool, bytes: u32) {
    // SAFETY: PRDBC is the header's second dword, found in a live DMA grant.
    unsafe { write_volatile(host(reach(clb, wide) + 4, 4) as *mut u32, bytes) };
}
