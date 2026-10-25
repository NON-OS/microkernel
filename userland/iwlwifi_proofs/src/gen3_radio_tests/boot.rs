// NONOS Operating System (AGPL-3.0-or-later)
//! The device masked once ALIVE is in, stopped after a failed boot, its
//! secure-boot status read, and every grant access bounded.


use crate::gen3::bringup::{boot, BootError};
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{Board, Memory};
use crate::gen3::prph::read_umac;
use crate::gen3::region::span;
use crate::gen3::regs::*;
use crate::gen3::start::stop_device;
use crate::gen3::ucode::Ucode;
use crate::gen3_model::Mem;
use super::rig::{SO_GF, booted, polls, rig};

#[test]
fn the_device_stops_signalling_interrupts_once_alive() {
    let r = rig();
    booted(&r, |_, _| {});
    assert_eq!(r.model.reg(CSR_INT_MASK), 0);
    assert_eq!(r.model.reg(CSR_MSIX_HW_INT_MASK_AD), 0xFFFF_FFFF);
    assert_eq!(r.model.reg(CSR_MSIX_FH_INT_MASK_AD), 0xFFFF_FFFF);
}

#[test]
fn a_failed_boot_leaves_the_device_stopped() {
    let r = rig();
    r.model.s.borrow_mut().no_alive = true;
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let mut c = polls();
    let board = Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None };
    assert_eq!(boot(&mut dev, &mut c, &mem, &layout, &ucode, &board), Err(BootError::NoAliveCause));
    let before = c.delays_us;
    assert!(stop_device(&r.model, &mut c), "the bus master reported stopped");
    let reset = r.model.reg(CSR_RESET);
    assert_eq!(reset & (RESET_STOP_MASTER | RESET_SW_RESET), RESET_STOP_MASTER | RESET_SW_RESET);
    assert_eq!(r.model.reg(CSR_INT_MASK), 0);
    assert_eq!(r.model.reg(CSR_MSIX_HW_INT_MASK_AD), 0xFFFF_FFFF);
    assert!(c.delays_us - before >= 6000, "the reset is given its settle time");
}

#[test]
fn secure_boot_status_is_read_through_the_umac_window() {
    let r = rig();
    r.model.s.borrow_mut().regs.insert(HBUS_TARG_PRPH_RDAT, 0x0000_0101);
    let mut c = polls();
    assert_eq!(read_umac(&r.model, &mut c, UMAG_SB_CPU_1_STATUS), Some(0x101));
    assert_eq!(r.model.reg(HBUS_TARG_PRPH_RADDR), 0x00D0_38C0 | 3 << 24, "24-bit address, byte enables");
}

#[test]
fn every_grant_access_is_bounded() {
    assert_eq!(span(4096, 0, 4096), Some(0..4096));
    assert_eq!(span(4096, 4095, 1), Some(4095..4096));
    assert_eq!(span(4096, 4095, 2), None, "one past the end");
    assert_eq!(span(4096, 4097, 0), None, "an empty read past the end");
    assert_eq!(span(4096, usize::MAX, 2), None, "the end overflows");
    assert_eq!(span(0, 0, 0), Some(0..0));
}
