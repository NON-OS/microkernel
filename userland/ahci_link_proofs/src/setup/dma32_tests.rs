// NONOS Operating System (AGPL-3.0-or-later)
//! Where a port's regions lie for the HBA's address width. An HBA without
//! CAP.S64A treats PxCLBU, PxFBU, CTBAU and DBAU as zero (AHCI 1.3.1, 3.1.1),
//! so the driver asks the broker for DMA32 then, and only then. The stand-in
//! broker puts a map without DMA32 above 4 GiB, so each test holds only if
//! the driver asks for DMA32 on the right HBA and the broker honours it: a
//! 32-bit HBA would reach no grant, and a 64-bit one would get low memory.

use super::give_back_tests::port_up;
use crate::constants::regs::{CAP_S64A, PORT_BASE, PORT_CLBU, PORT_FBU};
use crate::engine::region::DmaRegion;
use crate::engine::{retire, Port};
use crate::model::HbaRig;

/// Each region's bus address and length: list, table, FIS area, data.
fn spans(port: &Port) -> [(u64, u64); 4] {
    let span = |r: &DmaRegion| (r.device_addr(), r.len());
    [span(&port.clb), span(&port.ctba), span(&port._fb), span(&port.data)]
}

#[test]
fn an_hba_without_s64a_has_its_regions_below_4_gib() {
    let rig = HbaRig::left_armed();
    rig.hba().global.as_mut().expect("a whole HBA").cap &= !CAP_S64A;
    let (opened, port) = port_up(&rig);
    let spans = spans(&port);
    assert!(spans.iter().all(|&(at, n)| at + n <= 1 << 32), "past 4 GiB: {spans:x?}");
    retire(port, opened.regs);
}

#[test]
fn an_hba_with_s64a_reaches_its_regions_above_4_gib_through_each_upper_half() {
    let rig = HbaRig::left_armed();
    let (opened, port) = port_up(&rig);
    let spans = spans(&port);
    assert!(spans.iter().all(|&(at, _)| at >> 32 != 0), "below 4 GiB: {spans:x?}");
    let mut hba = rig.hba();
    let high = |at: u64| (at >> 32) as u32;
    assert_eq!(hba.read(PORT_BASE + PORT_CLBU), high(port.clb.device_addr()), "PxCLBU");
    assert_eq!(hba.read(PORT_BASE + PORT_FBU), high(port._fb.device_addr()), "PxFBU");
    drop(hba);
    retire(port, opened.regs);
}
