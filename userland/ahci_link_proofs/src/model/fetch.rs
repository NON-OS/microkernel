// NONOS Operating System (AGPL-3.0-or-later)
//! The HBA's reads of what the driver built in memory: slot 0's command
//! header, the command FIS and the PRD entry it points at, and its write of
//! the bytes moved back into the header (PRDBC).

use core::ptr::{read_volatile, write_volatile};

use super::script::Cmd;
use crate::engine::cmd_header::CmdHeader;
use crate::engine::cmd_table::PRDT_OFFSET;
use crate::engine::fis::FisH2D;
use crate::engine::prdt::PrdtEntry;

/// Where slot 0's PRD sends the data, and how many bytes it holds.
pub struct Slot {
    pub data: u64,
    pub bytes: u64,
}

fn header(clb: u64) -> CmdHeader {
    // SAFETY: `clb` is the rig's command list, a live allocation.
    unsafe { read_volatile(clb as *const CmdHeader) }
}

fn table(h: &CmdHeader) -> u64 {
    u64::from(h.ctba_low) | u64::from(h.ctba_high) << 32
}

pub fn command(clb: u64) -> Cmd {
    // SAFETY: the header names the rig's command table.
    let f = unsafe { read_volatile(table(&header(clb)) as *const FisH2D) };
    let lba = [f.lba5, f.lba4, f.lba3, f.lba2, f.lba1, f.lba0];
    let lba = lba.iter().fold(0u64, |a, &b| a << 8 | u64::from(b));
    Cmd { op: f.command, lba, count: u32::from(f.countl) | u32::from(f.counth) << 8 }
}

pub fn slot(clb: u64) -> Slot {
    let h = header(clb);
    if h.prdtl == 0 {
        return Slot { data: 0, bytes: 0 };
    }
    // SAFETY: the entry lies inside the rig's command table.
    let e = unsafe { read_volatile((table(&h) + PRDT_OFFSET as u64) as *const PrdtEntry) };
    let data = u64::from(e.dba_low) | u64::from(e.dba_high) << 32;
    Slot { data, bytes: u64::from(e.dbc & 0x3f_ffff) + 1 }
}

pub fn set_prdbc(clb: u64, bytes: u32) {
    // SAFETY: PRDBC is the header's second dword, in the rig's command list.
    unsafe { write_volatile((clb + 4) as *mut u32, bytes) };
}
