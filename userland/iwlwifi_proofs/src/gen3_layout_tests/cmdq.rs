// NONOS Operating System (AGPL-3.0-or-later)
//! A command goes out as its header and two buffers, the ring wraps, and a
//! UMAC register write uses its window.

use crate::gen3::cmdq::{CmdQueue, FIRST_TB};
use crate::gen3::plan::*;
use crate::gen3::prph::write_umac;
use crate::gen3::region::Region;
use crate::gen3::regs::*;
use crate::gen3::scan::SCAN_REQ_V17_LEN;
use crate::gen3_model::{Model, Polls};
use crate::regs::Mmio;
use super::regions::regions;

#[test]
fn a_command_goes_out_as_the_header_and_two_buffers_and_rings_the_doorbell() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl.clone(), rbs.clone());
    let mut q = CmdQueue::new();
    assert!(q.init(ctrl.as_ref()));
    let seq = q.send(&model, ctrl.as_ref(), 0x0C, 0x02, 0, &[0; 4]).unwrap();
    assert_eq!(seq, 0);
    assert_eq!(model.s.borrow().commands[0].payload, vec![0; 4], "decoded from the ring by the device");
    let mut t = [0u8; 256];
    ctrl.read(CMD_TFDS, &mut t);
    assert_eq!(u16::from_le_bytes([t[0], t[1]]), 1, "twelve bytes: one buffer");
    assert_eq!(u16::from_le_bytes([t[2], t[3]]), 12);
    let big = vec![0xA5; SCAN_REQ_V17_LEN];
    let seq = q.send(&model, ctrl.as_ref(), 1, 0x0D, 0, &big).unwrap();
    assert_eq!(seq, 1);
    ctrl.read(CMD_TFDS + 256, &mut t);
    assert_eq!(u16::from_le_bytes([t[0], t[1]]), 2);
    assert_eq!(u16::from_le_bytes([t[2], t[3]]) as usize, FIRST_TB);
    assert_eq!(u16::from_le_bytes([t[12], t[13]]) as usize, 8 + SCAN_REQ_V17_LEN - FIRST_TB);
    assert_eq!(model.s.borrow().commands[1].payload, big, "the device reassembles it whole");
    assert_eq!(model.reg(HBUS_TARG_WRPTR), 2, "queue 0, write pointer 2");
    assert!(q.send(&model, ctrl.as_ref(), 1, 0x0D, 0, &vec![0; CMD_BUF]).is_none(), "too large");
}

#[test]
fn the_command_ring_slot_wraps_at_128_and_the_write_pointer_at_16_bits() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl.clone(), rbs.clone());
    let mut q = CmdQueue::new();
    q.init(ctrl.as_ref());
    for i in 0..130u16 {
        let seq = q.send(&model, ctrl.as_ref(), 2, 3, 0, &[1, 2, 3, 4]).unwrap();
        assert_eq!(seq, i & 0xFF);
    }
    assert_eq!(model.reg(HBUS_TARG_WRPTR), 130, "the pointer does not wrap at the ring");
    assert_eq!(model.s.borrow().commands.len(), 130, "slots 0 and 1 reused cleanly");
}

#[test]
fn a_umac_register_write_uses_the_24_bit_window_and_releases_the_mac() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl, rbs);
    let mut c = Polls { per_wait: 4, delays_us: 0 };
    assert!(write_umac(&model, &mut c, UREG_CPU_INIT_RUN, 1));
    assert_eq!(model.reg(HBUS_TARG_PRPH_WADDR), 0x03D0_5C44, "address and byte enables");
    assert_eq!(model.read32(CSR_GP_CNTRL) & GP_MAC_ACCESS_REQ, 0, "access released");
}
