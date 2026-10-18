// NONOS Operating System (AGPL-3.0-or-later)
//! Which port reads armed before Bus Master: any one of PxCMD.ST, CR, FRE
//! and FR is enough and no other bit is, every port in PI among the first
//! `count` is read, not only the first, and no other port is, however wide
//! PI and the count are.

use super::armed_port;
use crate::constants::regs::{CMD_CR, CMD_FR, CMD_FRE, CMD_ST, PORT_BASE, PORT_CMD, PORT_STRIDE};
use crate::constants::regs::{PORT_CI, PORT_SACT};
use crate::model::HbaRig;
use crate::regs::Regs;

const ENGINES: u32 = CMD_ST | CMD_CR | CMD_FRE | CMD_FR;

/// A whole model HBA whose port 0 reads PxCMD `cmd`; its other ports read 0.
fn port0(cmd: u32) -> HbaRig {
    let rig = HbaRig::left_armed();
    rig.hba().reg.cmd = cmd;
    rig
}

#[test]
fn each_engine_bit_alone_reads_armed_and_no_other_bit_does() {
    let regs = Regs::new(0);
    for bit in [CMD_ST, CMD_CR, CMD_FRE, CMD_FR] {
        let _rig = port0(bit);
        assert_eq!(armed_port(regs, 1, 1), Some(0), "PxCMD {bit:#x}");
    }
    let _rig = port0(!ENGINES);
    assert_eq!(armed_port(regs, 1, 1), None);
}

#[test]
fn only_ports_in_pi_among_the_first_count_are_read() {
    let regs = Regs::new(0);
    let _rig = port0(ENGINES);
    // No port, or port 0 outside PI or past the count: nothing read armed.
    for (pi, count) in [(0, 32), (u32::MAX, 0), (!1, u8::MAX), (0, 0)] {
        assert_eq!(armed_port(regs, pi, count), None, "PI {pi:#x}, {count} ports");
    }
    // A count past the 32 ports AHCI has, with every PI bit set.
    for (pi, count) in [(1, 1), (u32::MAX, 32), (u32::MAX, u8::MAX)] {
        assert_eq!(armed_port(regs, pi, count), Some(0), "PI {pi:#x}, {count} ports");
    }
}

/// A register file in host memory for all 32 ports, every register clear
/// but port 3's at `off`, which reads `v`: no model is plugged in, so the
/// accessor reads this memory.
fn port3(off: u32, v: u32) -> Vec<u32> {
    let mut file = vec![0u32; (PORT_BASE + 32 * PORT_STRIDE) as usize / 4];
    file[(PORT_BASE + 3 * PORT_STRIDE + off) as usize / 4] = v;
    file
}

#[test]
fn a_slot_issued_or_queued_reads_armed_with_every_engine_off() {
    for (off, v) in [(PORT_CI, 1), (PORT_SACT, 1 << 31)] {
        let file = port3(off, v);
        assert_eq!(armed_port(Regs::new(file.as_ptr() as u64), 1 << 3, 4), Some(3), "{off:#x}");
    }
}

#[test]
fn a_port_past_the_first_in_pi_is_read_too() {
    let file = port3(PORT_CMD, CMD_FRE | CMD_FR);
    let regs = Regs::new(file.as_ptr() as u64);
    for pi in [1 << 3, 0b1001, 0b1101, 0b1010_1001, u32::MAX] {
        assert_eq!(armed_port(regs, pi, 4), Some(3), "PI {pi:#x}");
    }
    // Port 3 outside PI, or past the count.
    for (pi, count) in [(0b0111, 32), (!(1 << 3), 32), (0b1001, 3), (u32::MAX, 3)] {
        assert_eq!(armed_port(regs, pi, count), None, "PI {pi:#x}, {count} ports");
    }
}
