// NONOS Operating System (AGPL-3.0-or-later)
//! One frame as the controller runs it: FRINDEX on by eight micro-frames,
//! the periodic list from this frame's entry (a QH runs when its mask
//! names micro-frame 0), the async ring once round from ASYNCLISTADDR, and
//! a rung doorbell answered: the controller now holds no QH it was not
//! handed through the ring.

use super::exec::{run_qh, Pass};
use super::world::World;
use crate::facts::qh::LINK_T;
use crate::facts::regs::{CMD_ASE, CMD_IAAD, CMD_PSE, STS_IAA};

pub fn run(w: &mut World) {
    w.frindex = (w.frindex + 8) & 0x3FFF;
    if w.cmd & CMD_PSE != 0 && w.periodic != 0 {
        let entry = w.rd32(w.periodic + 4 * ((w.frindex >> 3) & 0x3FF));
        walk(w, entry, Pass::Periodic);
    }
    if w.cmd & CMD_ASE != 0 && w.async_addr != 0 {
        let start = w.async_addr;
        let mut at = start;
        for _ in 0..128 {
            run_qh(w, at, Pass::Async);
            at = w.rd32(at) & !0x1F;
            if at == start {
                break;
            }
        }
        assert_eq!(at, start, "the async ring does not close");
    }
    if w.doorbell {
        (w.doorbell, w.sts) = (false, w.sts | STS_IAA);
        w.cmd &= !CMD_IAAD;
    }
}

fn walk(w: &mut World, mut link: u32, pass: Pass) {
    for _ in 0..64 {
        if link & LINK_T != 0 {
            return;
        }
        let at = link & !0x1F;
        run_qh(w, at, pass);
        link = w.rd32(at);
    }
    panic!("the periodic list loops");
}
