// NONOS Operating System (AGPL-3.0-or-later)
//! One TD's transaction: its token decoded, the device on an enabled port
//! at that address and speed asked, and the TD's status written back as the
//! controller would. No device there is a timeout (CRC/timeout set).

use super::device::Reply;
use super::world::World;
use crate::facts::io::PORT_PE;
use crate::facts::td::*;

pub enum Outcome {
    Done,
    Halted,
    Nak,
}

const STATUS_NAK: u32 = 1 << 19;

pub fn transact(w: &mut World, td: u32) -> Outcome {
    let (ctrl, tok, buf) = (w.rd32(td + 4), w.rd32(td + 8), w.rd32(td + 12));
    let pid = tok as u8;
    let (addr, ep) = (((tok >> 8) & 0x7F) as u8, ((tok >> 15) & 0xF) as u8);
    let toggle = tok & TOKEN_TOGGLE != 0;
    let max = (((tok >> 21) + 1) & 0x7FF) as usize;
    let low = ctrl & CTRL_LS != 0;
    let out: Vec<u8> = if pid == PID_IN { vec![] } else { read(w, buf, max) };
    let port = (0..2).find(|&i| {
        w.portsc[i] & PORT_PE != 0
            && w.devs[i].as_ref().is_some_and(|d| d.addr == addr && d.low_speed == low)
    });
    let reply = match port {
        Some(i) => w.devs[i].as_mut().unwrap().packet(pid, ep, toggle, &out, max),
        None => return end(w, td, ctrl, STATUS_CRC_TIMEOUT),
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

fn read(w: &World, buf: u32, n: usize) -> Vec<u8> {
    if n == 0 {
        return vec![];
    }
    let at = (buf - super::world::BUS) as usize;
    w.mem[at..at + n].to_vec()
}

fn end(w: &mut World, td: u32, ctrl: u32, bits: u32) -> Outcome {
    w.wr32(td + 4, ctrl & !(STATUS_ACTIVE | STATUS_NAK) | bits);
    Outcome::Halted
}
