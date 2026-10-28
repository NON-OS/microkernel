// NONOS Operating System (AGPL-3.0-or-later)

//! The wrong layout on a BZ part fails where it would on silicon: V1 bits
//! never wake it, the V1 bus-master stop is never acknowledged, a
//! UREG_CPU_INIT_RUN kick starts nothing, ROM_START without the scratch
//! value starts nothing, and its software error is bit 5 of the MSI-X cause.

use super::rig::{polls, rig, run, GL};
use crate::gen3::bringup::BootError;
use crate::gen3::csr::{CsrLayout, CSR_FUNC_SCRATCH, FUNC_SCRATCH_INIT, GP_ROM_START};
use crate::gen3::dev::Dev;
use crate::gen3::prph::write_umac;
use crate::gen3::regs::{CSR_GP_CNTRL, CSR_INT, CSR_MSIX_HW_INT_CAUSES_AD, INT_BIT_ALIVE};
use crate::gen3::regs::{MSIX_HW_SW_ERR, UREG_CPU_INIT_RUN};
use crate::gen3::start::{start_hw, stop_device, StartError};
use crate::regs::Mmio;

#[test]
fn v1_bits_never_wake_a_bz_part() {
    let r = rig(GL, true);
    let (alive, _) = run(&r, CsrLayout::V1, &mut polls(), |_, _, _| ());
    assert_eq!(alive.err(), Some(BootError::Start(StartError::ClockNotReady)));
    assert_eq!(start_hw(&rig(GL, true).model, &mut polls(), CsrLayout::V2), Ok(()));
}

#[test]
fn only_the_v2_bus_master_stop_is_seen_stopped() {
    let r = rig(GL, true);
    assert!(!stop_device(&r.model, &mut polls(), CsrLayout::V1), "STOP_MASTER is not V2's");
    assert!(stop_device(&r.model, &mut polls(), CsrLayout::V2));
    let so = rig(GL, false);
    assert!(stop_device(&so.model, &mut polls(), CsrLayout::V1), "a V1 part as before");
}

#[test]
fn only_rom_start_after_the_scratch_value_starts_a_bz_part() {
    let r = rig(GL, true);
    let m = &r.model;
    start_hw(m, &mut polls(), CsrLayout::V2).unwrap();
    assert!(write_umac(m, &mut polls(), CsrLayout::V2, UREG_CPU_INIT_RUN, 1));
    m.write32(CSR_GP_CNTRL, m.read32(CSR_GP_CNTRL) | GP_ROM_START);
    assert_eq!(m.reg(CSR_INT) & INT_BIT_ALIVE, 0, "neither the UREG kick nor a bare ROM_START");
    m.write32(CSR_GP_CNTRL, m.read32(CSR_GP_CNTRL) & !GP_ROM_START);
    m.write32(CSR_FUNC_SCRATCH, FUNC_SCRATCH_INIT);
    m.write32(CSR_GP_CNTRL, m.read32(CSR_GP_CNTRL) | GP_ROM_START);
    assert_eq!(m.reg(CSR_INT) & INT_BIT_ALIVE, INT_BIT_ALIVE);
}

#[test]
fn a_bz_software_error_is_msix_bit_5() {
    let r = rig(GL, true);
    let (ctrl, rbs) = (r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let v1 = Dev::new(&r.model, ctrl, rbs);
    let v2 = Dev::new(&r.model, ctrl, rbs).with_csr(CsrLayout::V2);
    r.model.s.borrow_mut().regs.insert(CSR_MSIX_HW_INT_CAUSES_AD, 1 << 5);
    assert!(v2.error_cause() && !v1.error_cause(), "bit 5 is V2's alone");
    r.model.s.borrow_mut().regs.insert(CSR_MSIX_HW_INT_CAUSES_AD, MSIX_HW_SW_ERR);
    assert!(v1.error_cause() && !v2.error_cause(), "bit 25 is V1's alone");
}
