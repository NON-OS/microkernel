// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A modeled AX210-family device for the gen3 proofs. It keeps the registers
//! the start sequence touches, answers NIC-ready and clock-ready, decodes
//! peripheral writes through the HBUS window, and plays the firmware's side:
//! `UREG_CPU_INIT_RUN` raises the ALIVE cause and queues the ALIVE
//! notification, the receive write index lets queued packets into posted
//! buffers (completion descriptor, closed index), and a command doorbell is
//! decoded from the descriptor ring in DMA memory, logged, and answered by a
//! scripted responder. DMA regions are plain memory with a device address.
//!
//! With a transmit region attached (`TxModel`), the model also plays the
//! firmware's transmit side: SCD_QUEUE_CONFIG_CMD adds and removes queues
//! (recording each ring and byte count table), a doorbell on a queue other
//! than the command queue is decoded from that ring into the 802.11 frame
//! the host queued (checked against its byte count entry), answered with a
//! TX response, and handed to a scripted access point whose replies are
//! delivered as received frames.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::gen3::region::{Clock, Region};
use crate::gen3::regs::*;
use crate::regs::Mmio;

mod queues;
mod tx;
mod v2;

pub struct Mem {
    pub bytes: RefCell<Vec<u8>>,
    pub dev: u64,
}

impl Mem {
    pub fn new(len: usize, dev: u64) -> Rc<Self> {
        Rc::new(Self { bytes: RefCell::new(vec![0; len]), dev })
    }
}

impl Region for Mem {
    fn len(&self) -> usize {
        self.bytes.borrow().len()
    }
    fn dev(&self) -> u64 {
        self.dev
    }
    fn write(&self, off: usize, src: &[u8]) -> bool {
        let mut b = self.bytes.borrow_mut();
        match off.checked_add(src.len()) {
            Some(end) if end <= b.len() => {
                b[off..end].copy_from_slice(src);
                true
            }
            _ => false,
        }
    }
    fn read(&self, off: usize, dst: &mut [u8]) -> bool {
        let b = self.bytes.borrow();
        match off.checked_add(dst.len()) {
            Some(end) if end <= b.len() => {
                dst.copy_from_slice(&b[off..end]);
                true
            }
            _ => false,
        }
    }
}

/// A clock that re-checks a condition a fixed number of times. Its time is
/// `delays_us`: a settle adds its length, a wait that runs out adds the whole
/// wait, and a wait that is answered adds a millisecond.
pub struct Polls {
    pub per_wait: u32,
    pub delays_us: u64,
}

impl Clock for Polls {
    fn poll_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool {
        let met = (0..self.per_wait).any(|_| ready());
        self.delays_us += if met { 1000 } else { u64::from(ms) * 1000 };
        met
    }
    fn delay_us(&mut self, us: u32) {
        self.delays_us += us as u64;
    }
    fn now_ms(&mut self) -> u64 {
        self.delays_us / 1000
    }
}

/// A command the device saw: group, opcode, sequence, payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seen {
    pub group: u8,
    pub cmd: u8,
    pub seq: u16,
    pub payload: Vec<u8>,
}

/// One packet for the host: command header and payload.
#[derive(Clone, Debug)]
pub struct Out {
    pub cmd: u8,
    pub group: u8,
    pub seq: u16,
    pub payload: Vec<u8>,
}

pub type Responder = Box<dyn FnMut(&Seen) -> Vec<Out>>;

/// The scripted access point: given a frame the host transmitted, the
/// packets to deliver back.
pub type Air = Box<dyn FnMut(&[u8]) -> Vec<Out>>;

/// One frame the host queued: the queue, the frame, and the transmit
/// command's flags and rate word.
#[derive(Clone, Debug)]
pub struct Sent {
    pub queue: u16,
    pub frame: Vec<u8>,
    pub flags: u16,
    pub rate: u32,
}

/// The firmware's transmit side.
pub struct TxModel {
    pub mem: Rc<Mem>,
    /// Queue number to (ring, byte count table) device addresses.
    pub rings: HashMap<u16, (u64, u64)>,
    pub next_qid: u16,
    pub sent: Vec<Sent>,
    /// Frames whose byte count entry did not match their command's length.
    pub bc_mismatch: u32,
    pub air: Option<Air>,
    /// Answer a queue add with a reply of this many bytes instead of eight.
    pub queue_reply_len: usize,
    /// Answer no frame with a TX response.
    pub no_tx_response: bool,
}

impl TxModel {
    pub fn new(mem: Rc<Mem>) -> Self {
        Self {
            mem,
            rings: HashMap::new(),
            next_qid: 1,
            sent: Vec::new(),
            bc_mismatch: 0,
            air: None,
            queue_reply_len: 8,
            no_tx_response: false,
        }
    }
}

pub struct State {
    pub regs: HashMap<usize, u32>,
    pub prph: Vec<(u32, u32)>,
    pub commands: Vec<Seen>,
    pub queued: Vec<Out>,
    pub widx: usize,
    fr_read: usize,
    ur_write: usize,
    pub ready_never: bool,
    pub alive_status: u16,
    pub alive_sku: [u32; 3],
    pub no_alive: bool,
    pub responder: Option<Responder>,
    pub fw_error_after: Option<usize>,
    pub txq: Option<TxModel>,
    /// A frame delivered again each time the host looks for the interrupt
    /// cause: a channel that is never quiet.
    pub flood: Option<Out>,
    /// The device speaks the V2 (BZ) CSR layout, and only it.
    pub v2: bool,
    /// Every register write, in order.
    pub writes: Vec<(usize, u32)>,
}

#[derive(Clone)]
pub struct Model {
    pub s: Rc<RefCell<State>>,
    pub ctrl: Rc<Mem>,
    pub rbs: Rc<Mem>,
}

pub const RX_RING: usize = crate::gen3::plan::RX_RING;

impl Model {
    pub fn new(ctrl: Rc<Mem>, rbs: Rc<Mem>) -> Self {
        let mut regs = HashMap::new();
        regs.insert(CSR_GP_CNTRL, GP_HW_RF_KILL_SW);
        let s = State {
            regs,
            prph: Vec::new(),
            commands: Vec::new(),
            queued: Vec::new(),
            widx: 0,
            fr_read: 0,
            ur_write: 0,
            ready_never: false,
            alive_status: 0xCAFE,
            alive_sku: [0; 3],
            no_alive: false,
            responder: None,
            fw_error_after: None,
            txq: None,
            flood: None,
            v2: false,
            writes: Vec::new(),
        };
        Self { s: Rc::new(RefCell::new(s)), ctrl, rbs }
    }

    pub fn reg(&self, off: usize) -> u32 {
        *self.s.borrow().regs.get(&off).unwrap_or(&0)
    }

    pub fn queue(&self, out: Out) {
        self.s.borrow_mut().queued.push(out);
        self.deliver();
    }

    // Move queued packets into buffers the host has posted.
    fn deliver(&self) {
        let mut s = self.s.borrow_mut();
        while !s.queued.is_empty() {
            let stocked = s.widx.wrapping_sub(s.fr_read) & (RX_RING - 1);
            if stocked == 0 {
                break;
            }
            let out = s.queued.remove(0);
            let fr = crate::gen3::plan::RX_FREE + s.fr_read * 16;
            let mut d = [0u8; 16];
            self.ctrl.read(fr, &mut d);
            let vid = u16::from_le_bytes([d[0], d[1]]);
            let addr = u64::from_le_bytes(d[8..16].try_into().unwrap());
            let off = (addr - self.rbs.dev) as usize;
            let mut pkt = Vec::new();
            let len = (4 + out.payload.len()) as u32;
            pkt.extend_from_slice(&len.to_le_bytes());
            pkt.push(out.cmd);
            pkt.push(out.group);
            pkt.extend_from_slice(&out.seq.to_le_bytes());
            pkt.extend_from_slice(&out.payload);
            self.rbs.write(off, &pkt);
            // `iwl_rx_completion_desc` (rbid at 4), or the BZ one (rbid at 0).
            let (size, at) = if s.v2 { (4, 0) } else { (32, 4) };
            let mut cd = vec![0u8; size];
            cd[at..at + 2].copy_from_slice(&vid.to_le_bytes());
            self.ctrl.write(crate::gen3::plan::RX_USED + s.ur_write * size, &cd);
            s.fr_read = (s.fr_read + 1) & (RX_RING - 1);
            s.ur_write = (s.ur_write + 1) & (RX_RING - 1);
            let closed = s.ur_write as u16;
            self.ctrl.write(crate::gen3::plan::RB_STTS, &closed.to_le_bytes());
        }
    }

    fn prph_write(&self, addr: u32, val: u32) {
        self.s.borrow_mut().prph.push((addr, val));
        if addr == UREG_CPU_INIT_RUN + 0x30_0000 && val == 1 && !self.s.borrow().v2 {
            self.raise_alive();
        }
        if addr == UREG_DOORBELL_TO_ISR6 + 0x30_0000 && val & UREG_DOORBELL_TO_ISR6_PNVM != 0 {
            self.queue(Out { cmd: 0xFE, group: 0x0C, seq: 0x8000, payload: vec![0; 4] });
        }
    }

    // Decode the command whose doorbell was just rung and answer it.
    fn doorbell(&self, wptr: u32) {
        let slot = (wptr.wrapping_sub(1) as usize) & 127;
        let tfd = crate::gen3::plan::CMD_TFDS + slot * 256;
        let bytes: Vec<u8> = queues::buffers(&self.ctrl, tfd).concat();
        let seen = Seen {
            cmd: bytes[0],
            group: bytes[1],
            seq: u16::from_le_bytes([bytes[2], bytes[3]]),
            payload: bytes[8..].to_vec(),
        };
        let fail = {
            let mut s = self.s.borrow_mut();
            s.commands.push(seen.clone());
            s.fw_error_after.is_some_and(|k| s.commands.len() > k)
        };
        if fail {
            *self.s.borrow_mut().regs.entry(CSR_INT).or_insert(0) |= INT_BIT_SW_ERR;
            return;
        }
        if let Some(out) = self.queue_config(&seen) {
            self.queue(out);
            return;
        }
        let responder = self.s.borrow_mut().responder.take();
        let outs = match responder {
            Some(mut r) => {
                let o = r(&seen);
                self.s.borrow_mut().responder = Some(r);
                o
            }
            None => vec![reply(&seen, vec![])],
        };
        for o in outs {
            self.queue(o);
        }
    }
}

/// The plain reply to a command: same opcode, group and sequence.
pub fn reply(c: &Seen, payload: Vec<u8>) -> Out {
    Out { cmd: c.cmd, group: c.group, seq: c.seq, payload }
}

impl Mmio for Model {
    fn read32(&self, off: usize) -> u32 {
        // A wait that found nothing reads the interrupt cause: on a flooded
        // channel the next frame has arrived by then.
        let flood = if off == CSR_INT { self.s.borrow().flood.clone() } else { None };
        if let Some(f) = flood {
            self.queue(f);
        }
        let s = self.s.borrow();
        let v = *s.regs.get(&off).unwrap_or(&0);
        match off {
            CSR_HW_IF_CONFIG_REG if s.ready_never => v & !HW_IF_CONFIG_NIC_READY,
            CSR_GP_CNTRL if s.v2 => v2::gp_read(v),
            CSR_GP_CNTRL if v & (GP_INIT_DONE | GP_MAC_ACCESS_REQ) != 0 => v | GP_MAC_CLOCK_READY,
            _ => v,
        }
    }
    fn write32(&self, off: usize, val: u32) {
        let v2 = {
            let mut s = self.s.borrow_mut();
            s.writes.push((off, val));
            s.v2
        };
        match off {
            CSR_GP_CNTRL | HBUS_TARG_WRPTR if v2 && self.v2_write(off, val) => {}
            CSR_INT | CSR_MSIX_HW_INT_CAUSES_AD => {
                let mut s = self.s.borrow_mut();
                let e = s.regs.entry(off).or_insert(0);
                *e &= !val;
            }
            HBUS_TARG_PRPH_WDAT => {
                let addr = self.reg(HBUS_TARG_PRPH_WADDR) & 0x00FF_FFFF;
                self.prph_write(addr, val);
            }
            RFH_Q0_FRBDCB_WIDX_TRG => {
                self.s.borrow_mut().widx = val as usize;
                self.deliver();
            }
            HBUS_TARG_WRPTR => {
                self.s.borrow_mut().regs.insert(off, val);
                match (val >> 16) as u16 {
                    0 => self.doorbell(val & 0xFFFF),
                    q => self.tx_doorbell(q, val & 0xFFFF),
                }
            }
            // Stopping the bus master is acknowledged at once.
            CSR_RESET if val & RESET_STOP_MASTER != 0 && !v2 => {
                self.s.borrow_mut().regs.insert(off, val | RESET_MASTER_DISABLED);
            }
            _ => {
                self.s.borrow_mut().regs.insert(off, val);
            }
        }
    }
}
