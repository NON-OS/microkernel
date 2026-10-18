// NONOS Operating System (AGPL-3.0-or-later)
//! `program` points a port at its own command list and FIS area before FIS
//! receive goes on. On a port a dead driver left armed, the HBA reset stops
//! the engines but leaves PxCLB and PxFB on that driver's frames (AHCI
//! 1.3.1, 10.4.3), so FRE set before PxFB is written would post received
//! FISes into a frame the kernel has freed.

use super::port::Port;
use super::program::program;
use crate::constants::regs::{CAP_S64A, PORT_BASE};
use crate::controller::enable_ahci;
use crate::model::HbaRig;
use crate::regs::Regs;

#[test]
fn fis_receive_goes_on_only_with_fb_at_the_ports_own_area() {
    let rig = HbaRig::left_armed();
    let regs = Regs::new(0);
    enable_ahci(regs).expect("the HBA resets");
    let dead = (rig.left[0].addr(), rig.left[1].addr());
    let hba = rig.hba();
    assert_eq!((hba.clb, hba.ptr.fb), dead, "the reset keeps PxCLB and PxFB");
    drop(hba);
    let port = Port::map(9, 3, PORT_BASE, CAP_S64A).expect("the port's regions");
    program(regs, &port);
    let hba = rig.hba();
    let own = port._fb.device_addr();
    assert_eq!(hba.ptr.fb_at_fre, [own], "FRE went on with PxFB elsewhere than {own:#x}");
    assert_eq!(hba.clb, port.clb.device_addr(), "PxCLB names the port's command list");
    assert!(hba.ptr.clb_at_st.is_empty(), "the engine is start's to turn on");
}
