// NONOS Operating System (AGPL-3.0-or-later)
//! What the chip does when a register is written: self-clearing request
//! bits, the efuse and XTAL SI accesses, the firmware CPU's start, the
//! mailbox, the RF serial interface, and the DMA engine's kicks.

use super::state::State;
use super::{chans, cpu, dbi, mailbox, rf, rxq, txq, xtal};

/// Clear `bits` of word `a` as the hardware does once a request is taken.
fn done(s: &mut State, a: u32, bits: u32) {
    let v = s.word(a) & !bits;
    s.regs.insert(a, v);
}

fn nctl(s: &mut State, v: u32) {
    s.regs.insert(0x1bff8, v);
}

pub fn wrote(s: &mut State, a: u32, old: u32, new: u32, mask: u32) {
    match a {
        0x4 => {
            if new & 0x100 != 0 {
                s.powered = true; // B_AX_APFN_ONMAC: the MAC powers on
            }
            if new & 0x200 != 0 {
                s.powered = false; // B_AX_APFM_OFFMAC: the CPU loses its image
                s.fwdl = cpu::Fwdl::Off;
            }
            done(s, a, 0x300);
        }
        0x30 if new & (1 << 29) == 0 => xtal::efuse(s, new),
        0x88 => cpu::platform(s, old, new),
        0x270 => xtal::si(s, new),
        0x1090 if mask & 0xff_0000 != 0 => dbi::take(s, new),
        0x10a0 => done(s, a, 0x300), // MDIO flags
        0x1000 => {
            done(s, a, 0x8); // B_AX_RST_BDRAM
            txq::pump(s);
        }
        0x1010 => txq::pump(s),
        0x1014 | 0x1018 => chans::clear(s, a, new),
        0x1058 | 0x1078 | 0x1080 => txq::pump(s),
        0x1050 | 0x1054 => rxq::released(s, a),
        0x8160 if new & 0xff == 1 => mailbox::take(s),
        0xce34 => done(s, a, 0x100), // address CAM reset
        0xce3c => done(s, a, 0x3),   // RESPBA CAM reset
        0xdaf0 => xtal::lte(s, new),
        0x10370 => rf::swsi_write(s, new),
        0x10378 => rf::swsi_read(s, new),
        // An NCTL one-shot (command + 1) reports done (0x55); clearing
        // R_NCTL_N1 clears the report for the next one.
        0x18000 if new & 0xf == 0x9 => nctl(s, 0x55),
        0x18010 if new & 0xff == 0 => nctl(s, 0),
        _ => {}
    }
}
