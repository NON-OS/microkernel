// NONOS Operating System (AGPL-3.0-or-later)
//! A qTD run in packets of the QH's size from its buffer pointer, each
//! toggle the qTD's (DTC) or the QH's, for the high-speed device at the
//! QH's address on an enabled port. A NAK leaves it where it got to; a
//! STALL or no answer halts qTD and QH; a retired qTD moves the overlay on,
//! to its alternate after a short IN packet when it names one (retire.rs).

use super::device::Reply;
use super::pid::{PID_IN, PID_OUT, PID_SETUP};
use super::retire::{done, halt, stay};
use super::world::{Speed, World, BUS};
use crate::facts::port::PORT_PE;
use crate::facts::qh::{CH_DTC, QH_TOKEN};
use crate::facts::qtd::TOK_DT;

/// Run qTD `q` on `qh`; whether it retired cleanly.
pub fn packets(w: &mut World, qh: u32, q: u32) -> bool {
    let ch = w.rd32(qh + 4);
    let (fa, ep, mps) = ((ch & 0x7F) as u8, ((ch >> 8) & 0xF) as u8, ((ch >> 16) & 0x7FF) as usize);
    let tok = w.rd32(q + 8);
    let pid = [PID_OUT, PID_IN, PID_SETUP][((tok >> 8) & 3).min(2) as usize];
    let dtc = ch & CH_DTC != 0;
    let mut dt = if dtc { tok } else { w.rd32(qh + QH_TOKEN as u32) } & TOK_DT != 0;
    let Some(i) = (0..w.devs.len()).find(|&i| {
        w.ports[i] & PORT_PE != 0
            && w.speed[i] == Speed::High
            && w.devs[i].as_ref().is_some_and(|d| d.addr == fa)
    }) else {
        return halt(w, qh, q, 0x08);
    };
    let (mut left, mut ptr) = (((tok >> 16) & 0x7FFF) as usize, w.rd32(q + 12));
    loop {
        let max = left.min(mps);
        let at = (ptr.max(BUS) - BUS) as usize;
        let out = if pid == PID_IN { vec![] } else { w.mem[at..at + max].to_vec() };
        let n = match w.devs[i].as_mut().unwrap().packet(pid, ep, dt, &out, max) {
            Reply::Nak => return stay(w, qh, q, left, ptr, dt, dtc),
            Reply::Stall => return halt(w, qh, q, 0),
            Reply::Ack(d) if d.len() > max => return halt(w, qh, q, 0x10),
            Reply::Ack(d) if pid == PID_IN => {
                w.mem[at..at + d.len()].copy_from_slice(&d);
                d.len()
            }
            Reply::Ack(_) => max,
        };
        dt = !dt;
        (left, ptr) = (left - n, ptr + n as u32);
        let short = pid == PID_IN && n < max;
        if left == 0 || short {
            return done(w, qh, q, left, dt, dtc, short);
        }
    }
}
