// NONOS Operating System (AGPL-3.0-or-later)
//! One general TD run for its ED's device (bus.rs): packets of the ED's size from
//! CBP to BE, each toggle checked by the device; a NAK leaves the TD where
//! it got to, a short IN packet ends it (an error unless rounding is on),
//! a STALL or no answer retires it in error (retire.rs).

use super::bus::{find, path};
use super::device::Reply;
use super::pid::{PID_IN, PID_OUT, PID_SETUP};
use super::retire::retire;
use super::world::World;
use crate::facts::ed::{ED_S, HEAD_C};
use crate::facts::port::PORT_PES;
use crate::facts::td::{CC_STALL, TD_R};

#[derive(PartialEq, Eq, Debug)]
pub enum Step {
    Retired,
    Nak,
    Halted,
}

const CC_NOT_RESPONDING: u32 = 5;
const CC_OVERRUN: u32 = 8;
const CC_UNDERRUN: u32 = 9;

pub fn run(w: &mut World, ed: u32, td: u32) -> Step {
    let (word, ctl) = (w.rd32(ed), w.rd32(td));
    let (fa, en, mps) = ((word & 0x7F) as u8, ((word >> 7) & 0xF) as u8, (word >> 16) & 0x7FF);
    let pid = [PID_SETUP, PID_OUT, PID_IN][((ctl >> 19) & 3).min(2) as usize];
    let t = (ctl >> 24) & 3;
    let mut toggle = if t & 2 != 0 { t & 1 != 0 } else { w.rd32(ed + 8) & HEAD_C != 0 };
    let low = word & ED_S != 0;
    // Enabled port or below a hub there; low speed only by an ED saying so.
    let found = (0..w.devs.len()).find_map(|i| {
        let d = w.devs[i].as_ref().filter(|_| w.ports[i] & PORT_PES != 0)?;
        let speed = if d.low_speed { 2 } else { 1 };
        path(d, speed, fa).filter(|p| (p.speed == 2) == low).map(|_| (i, speed))
    });
    let Some((i, speed)) = found else {
        return retire(w, ed, td, CC_NOT_RESPONDING, None);
    };
    let (mut cbp, be) = (w.rd32(td + 4), w.rd32(td + 12));
    loop {
        let left = if cbp == 0 { 0 } else { (be - cbp + 1) as usize };
        let max = left.min(mps as usize);
        let at = (cbp.max(super::world::BUS) - super::world::BUS) as usize;
        let out = if pid == PID_IN { vec![] } else { w.mem[at..at + max].to_vec() };
        let dev = find(w.devs[i].as_mut().unwrap(), speed, fa).unwrap().0;
        let n = match dev.packet(pid, en, toggle, &out, max) {
            Reply::Nak => {
                w.wr32(td + 4, cbp);
                w.wr32(td, ctl & !(3 << 24) | (2 | toggle as u32) << 24);
                return Step::Nak;
            }
            Reply::Stall => return retire(w, ed, td, CC_STALL, None),
            Reply::Ack(d) if d.len() > max => return retire(w, ed, td, CC_OVERRUN, None),
            Reply::Ack(d) if pid == PID_IN => {
                w.mem[at..at + d.len()].copy_from_slice(&d);
                d.len()
            }
            Reply::Ack(_) => max,
        };
        toggle = !toggle;
        let short = pid == PID_IN && n < max;
        cbp = if left == n { 0 } else { cbp + n as u32 };
        w.wr32(td + 4, cbp);
        if short && ctl & TD_R == 0 {
            return retire(w, ed, td, CC_UNDERRUN, None);
        }
        if cbp == 0 || short {
            return retire(w, ed, td, 0, Some(toggle));
        }
    }
}
