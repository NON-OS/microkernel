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

//! A BusLogic MultiMaster adapter as VirtualBox's DevBusLogic answers the
//! driver: the status, command/data and interrupt registers, the hard reset
//! and its diagnostics, the commands the driver sends with their parameters
//! and replies, and the extended mailboxes, each CCB the adapter takes run
//! on `nonos_scsi`'s simulated units and its completion posted in the
//! incoming mailbox.

#[path = "../../nonos_scsi/src/sim.rs"]
pub mod sim;

use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use nonos_libc::{dma_at, install_ports, Ports};
use sim::{check, Answer, Unit};

pub const DACT: u8 = 0x80;
pub const DFAIL: u8 = 0x40;
pub const INREQ: u8 = 0x20;
pub const HARDY: u8 = 0x10;
pub const DIRRDY: u8 = 0x04;
pub const CMDINV: u8 = 0x01;
pub const CMDC: u8 = 0x04;
pub const MBIF: u8 = 0x01;
pub const ANYINTR: u8 = 0x80;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ccb {
    pub opcode: u8,
    pub direction: u8,
    pub cdb: Vec<u8>,
    pub sense_len: u8,
    pub data_len: u32,
    pub data: u32,
    pub target: u8,
    pub lun: u8,
    pub sense: u32,
}

pub struct State {
    pub units: BTreeMap<(u16, u16), Unit>,
    pub status: u8,
    pub interrupt: u8,
    pub op: Option<u8>,
    pub params: Vec<u8>,
    pub reply: VecDeque<u8>,
    pub mbox: Option<(u32, u8)>,
    pub host_id: u8,
    /// Every command and its parameters, in order; a hard reset is 0xFF.
    pub log: Vec<(u8, Vec<u8>)>,
    pub ccbs: Vec<Ccb>,
    pub resets: u32,
    /// Knobs.
    pub diag_fail: bool,
    pub no_disable_command: bool,
    pub refuse_mailboxes: bool,
    /// INITIALIZE EXTENDED MAILBOX completes without taking them.
    pub ignore_mailboxes: bool,
    pub stall: bool,
    pub host: Option<u8>,
    pub completion: Option<u8>,
    pub wrong_ccb: bool,
    /// An underrun is reported as host status 12h rather than success.
    pub underrun_status: bool,
}

pub type Shared = Rc<RefCell<State>>;

struct Hook(Shared);

impl Ports for Hook {
    fn read(&mut self, grant: u64, offset: u16, width: u8) -> u32 {
        assert_eq!((grant, width), (50, 1), "byte reads of the port window");
        let mut s = self.0.borrow_mut();
        match offset {
            0 => s.status.into(),
            1 => {
                let b = s.reply.pop_front().expect("a data in read with no reply byte");
                if s.reply.is_empty() {
                    s.status &= !DIRRDY;
                }
                b.into()
            }
            2 => s.interrupt.into(),
            _ => panic!("read of port {offset}"),
        }
    }

    fn write(&mut self, grant: u64, offset: u16, width: u8, value: u32) {
        assert_eq!((grant, width), (50, 1), "byte writes of the port window");
        let mut s = self.0.borrow_mut();
        match offset {
            0 => s.control(value as u8),
            1 => s.command_byte(value as u8),
            _ => panic!("write of port {offset}"),
        }
    }
}

pub fn install(units: Vec<((u16, u16), Unit)>) -> Shared {
    let s = Rc::new(RefCell::new(State {
        units: units.into_iter().collect(),
        status: HARDY,
        interrupt: 0,
        op: None,
        params: Vec::new(),
        reply: VecDeque::new(),
        mbox: None,
        host_id: 7,
        log: Vec::new(),
        ccbs: Vec::new(),
        resets: 0,
        diag_fail: false,
        no_disable_command: false,
        refuse_mailboxes: false,
        ignore_mailboxes: false,
        stall: false,
        host: None,
        completion: None,
        wrong_ccb: false,
        underrun_status: false,
    }));
    install_ports(Box::new(Hook(s.clone())));
    s
}

fn rd<T: Copy>(bus: u32) -> T {
    // SAFETY: the driver names bus addresses in its DMA grant; dma_at checks
    // they are in one and returns the host memory behind them.
    unsafe { std::ptr::read_unaligned(dma_at(bus as u64) as *const T) }
}

fn wr<T: Copy>(bus: u32, v: T) {
    // SAFETY: as above.
    unsafe { std::ptr::write_unaligned(dma_at(bus as u64) as *mut T, v) }
}

impl State {
    fn control(&mut self, v: u8) {
        if v & 0x80 != 0 {
            self.resets += 1;
            self.log.push((0xFF, vec![]));
            self.mbox = None;
            self.op = None;
            self.reply.clear();
            self.interrupt = 0;
            self.stall = false;
            self.status = if self.diag_fail { DFAIL | HARDY } else { INREQ | HARDY };
        }
        if v & 0x20 != 0 {
            self.interrupt = 0;
        }
    }

    fn command_byte(&mut self, b: u8) {
        let Some(op) = self.op else {
            self.status &= !CMDINV;
            self.op = Some(b);
            self.params.clear();
            return self.maybe_run();
        };
        let _ = op;
        self.params.push(b);
        self.maybe_run();
    }

    fn maybe_run(&mut self) {
        let op = self.op.unwrap();
        let need = match op {
            0x02 | 0x0B => 0,
            0x25 if !self.no_disable_command => 1,
            0x81 => 5,
            _ => {
                self.log.push((op, vec![]));
                self.op = None;
                self.status |= CMDINV;
                self.interrupt |= CMDC | ANYINTR;
                return;
            }
        };
        if self.params.len() < need {
            return;
        }
        self.op = None;
        let params = std::mem::take(&mut self.params);
        self.log.push((op, params.clone()));
        match op {
            0x02 => return self.start_mailbox(),
            0x0B => {
                self.reply.extend([0, 0, self.host_id]);
                self.status |= DIRRDY;
            }
            0x81 => {
                if self.refuse_mailboxes || params[0] == 0 {
                    self.status |= CMDINV;
                } else if !self.ignore_mailboxes {
                    let base = u32::from_le_bytes(params[1..5].try_into().unwrap());
                    self.mbox = Some((base, params[0]));
                    self.status &= !INREQ;
                }
            }
            _ => {}
        }
        self.interrupt |= CMDC | ANYINTR;
    }

    fn start_mailbox(&mut self) {
        let Some((base, count)) = self.mbox else { return };
        if self.stall {
            return;
        }
        for i in 0..count as u32 {
            let out = base + i * 8;
            if rd::<u8>(out + 7) != 1 {
                continue;
            }
            let ccb_at: u32 = rd(out);
            wr::<u8>(out + 7, 0);
            let (host, target, completion) = self.run_ccb(ccb_at);
            let inb = base + count as u32 * 8;
            assert_eq!(rd::<u8>(inb + 7), 0, "the incoming mailbox was left full");
            wr(inb, if std::mem::take(&mut self.wrong_ccb) { ccb_at + 0x40 } else { ccb_at });
            wr::<u8>(inb + 4, host);
            wr::<u8>(inb + 5, target);
            wr::<u8>(inb + 7, completion);
            self.interrupt |= MBIF | ANYINTR;
        }
    }

    /// The adapter status, the target status and the completion code.
    fn run_ccb(&mut self, at: u32) -> (u8, u8, u8) {
        let c = Ccb {
            opcode: rd(at),
            direction: rd(at + 1),
            cdb: (0..rd::<u8>(at + 2) as u32).map(|i| rd(at + 18 + i)).collect(),
            sense_len: rd(at + 3),
            data_len: rd(at + 4),
            data: rd(at + 8),
            target: rd(at + 16),
            lun: rd(at + 17),
            sense: rd(at + 36),
        };
        self.ccbs.push(c.clone());
        if let Some(h) = self.host.take() {
            return (h, 0, self.completion.take().unwrap_or(4));
        }
        if let Some(code) = self.completion.take() {
            return (0, 0, code);
        }
        assert_eq!(c.opcode, 0x03, "an initiator CCB with residual");
        assert!(c.cdb.len() <= 12);
        let dir = c.direction >> 3 & 3;
        assert!(matches!(dir, 1..=3), "a checked direction");
        let (t, l) = (c.target as u16, c.lun as u16);
        if t == self.host_id as u16 || !self.units.keys().any(|k| k.0 == t) {
            return (0x11, 0, 4);
        }
        let out: Vec<u8> =
            if dir == 2 { (0..c.data_len).map(|i| rd(c.data + i)).collect() } else { vec![] };
        if dir == 3 {
            assert_eq!(c.data_len, 0);
        }
        let a = match self.units.get_mut(&(t, l)) {
            Some(u) => u.execute(&c.cdb, &out),
            None if c.cdb[0] == 0x12 => {
                let mut d = vec![0u8; 36];
                d[0] = 0x7F;
                Answer { status: 0, sense: vec![], data: d }
            }
            None => check(0x5, 0x25, 0x00),
        };
        let moved = if dir == 1 {
            let n = a.data.len().min(c.data_len as usize);
            for (i, &b) in a.data[..n].iter().enumerate() {
                wr(c.data + i as u32, b);
            }
            n as u32
        } else {
            c.data_len
        };
        let residual = c.data_len - moved;
        wr(at + 4, residual);
        for (i, &b) in a.sense.iter().take(c.sense_len as usize).enumerate() {
            wr(c.sense + i as u32, b);
        }
        let host = if residual > 0 && self.underrun_status { 0x12 } else { 0 };
        wr::<u8>(at + 14, host);
        wr::<u8>(at + 15, a.status);
        (host, a.status, if a.status == 0 && host == 0 { 1 } else { 4 })
    }
}
