// NONOS Operating System (AGPL-3.0-or-later)
//! One TD's transaction: its token decoded, the device at that address and
//! speed asked (on an enabled port or below a hub there, bus.rs), and the
//! TD's status written back as the controller would. No device there is a
//! timeout (CRC/timeout set).

use super::bus::{find, path};
use super::device::Reply;
use super::td_io::{end, read};
use super::world::World;
use crate::facts::io::PORT_PE;
use crate::facts::td::*;

pub enum Outcome {
    Done,
    Halted,
    Nak,
}

pub(super) const STATUS_NAK: u32 = 1 << 19;

pub fn transact(w: &mut World, td: u32) -> Outcome {
    let (ctrl, tok, buf) = (w.rd32(td + 4), w.rd32(td + 8), w.rd32(td + 12));
    let pid = tok as u8;
    let (addr, ep) = (((tok >> 8) & 0x7F) as u8, ((tok >> 15) & 0xF) as u8);
    let toggle = tok & TOKEN_TOGGLE != 0;
    let max = (((tok >> 21) + 1) & 0x7FF) as usize;
    let low = ctrl & CTRL_LS != 0;
    let out: Vec<u8> = if pid == PID_IN { vec![] } else { read(w, buf, max) };
    let found = (0..2).find_map(|i| {
        let d = w.devs[i].as_ref().filter(|_| w.portsc[i] & PORT_PE != 0)?;
        let speed = if d.low_speed { 2 } else { 1 };
        path(d, speed, addr).map(|p| (i, speed, p))
    });
    let reply = match found {
        // A low-speed device, on the root port or below a full-speed hub,
        // is reached only by a TD that says so (the preamble).
        Some((i, speed, p)) if (p.speed == 2) == low => {
            let dev = find(w.devs[i].as_mut().unwrap(), speed, addr).unwrap().0;
            dev.packet(pid, ep, toggle, &out, max)
        }
        _ => return end(w, td, ctrl, STATUS_CRC_TIMEOUT),
    };
    let n = match reply {
        Reply::Nak => {
            w.wr32(td + 4, ctrl | STATUS_NAK);
            return Outcome::Nak;
        }
        Reply::Stall => return end(w, td, ctrl, STATUS_STALLED),
        Reply::Ack(data) if data.len() > max => return end(w, td, ctrl, STATUS_BABBLE),
        Reply::Ack(data) if pid == PID_IN && !data.is_empty() => {
            let at = (buf - super::world::BUS) as usize;
            w.mem[at..at + data.len()].copy_from_slice(&data);
            data.len()
        }
        Reply::Ack(data) if pid == PID_IN => data.len(),
        Reply::Ack(_) => max,
    };
    let actlen = if n == 0 { ACTLEN_MASK } else { n as u32 - 1 };
    let done = ctrl & !(STATUS_ACTIVE | STATUS_NAK | ACTLEN_MASK) | actlen;
    w.wr32(td + 4, done);
    if pid == PID_IN && n < max && ctrl & CTRL_SPD != 0 {
        return Outcome::Halted;
    }
    Outcome::Done
}
