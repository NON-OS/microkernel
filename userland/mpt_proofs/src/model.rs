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

//! An MPI 1.x IOC as QEMU's mptsas and VirtualBox's LsiLogic answer the
//! driver: the doorbell with the IOC's state, the handshake (the host's
//! start word taken with a doorbell interrupt, each message dword acked, the
//! reply sixteen bits per interrupt and one more interrupt to end it), the
//! message unit reset, IOC FACTS and IOC INIT; then the request FIFO, the
//! reply free FIFO the host fills and the reply post FIFO it reads: a
//! context reply for a request that went well, an address reply in a free
//! frame otherwise.

#[path = "../../nonos_scsi/src/sim.rs"]
pub mod sim;

use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use nonos_libc::dma_at;
use sim::{check, Answer, Unit};

use crate::regs::Regs;

pub const READY: u32 = 0x1000_0000;
pub const OPERATIONAL: u32 = 0x2000_0000;
const ACTIVE: u32 = 0x0800_0000;
const DB_INT: u32 = 0x1;
const IOP_DB: u32 = 0x8000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Idle,
    /// The start word was taken. Its interrupt comes on the host's second
    /// look, and the host clears it then, not before.
    Started {
        looks: u8,
    },
    Collecting,
    Replying,
    /// The last word was taken; the end interrupt comes on the host's next
    /// look, and the handshake is over once the host clears it.
    Ending {
        raised: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Io {
    pub target: u8,
    pub bus: u8,
    pub cdb: Vec<u8>,
    pub sense_len: u8,
    pub context: u32,
    pub lun: [u8; 8],
    pub control: u32,
    pub data_len: u32,
    pub sense_addr: u32,
    pub sge_flags_len: u32,
    pub sge_addr: u32,
}

pub struct State {
    pub units: BTreeMap<(u16, u16), Unit>,
    pub state: u32,
    pub his: u32,
    pub mask: u32,
    phase: Phase,
    want: usize,
    msg: Vec<u32>,
    reply: VecDeque<u16>,
    pub free: VecDeque<u32>,
    pub post: VecDeque<u32>,
    /// Every handshake message's function and every MUR, in order (0x40).
    pub log: Vec<u8>,
    pub init: Option<Vec<u8>>,
    pub ios: Vec<Io>,
    /// Knobs.
    pub max_devices: u8,
    pub frame_dwords: u16,
    pub short_facts: bool,
    pub refuse_init: bool,
    pub stuck: bool,
    pub stall: bool,
    pub iocstatus: Option<u16>,
    pub wrong_context: bool,
    pub port_enable_address_reply: bool,
    /// IOC INIT answered as done while the IOC stays READY.
    pub init_not_operational: bool,
    /// Sense written to the buffer with AUTOSENSE_VALID left clear.
    pub sense_unflagged: bool,
    /// Another host's handshake holds the doorbell.
    pub doorbell_busy: bool,
}

#[derive(Clone)]
pub struct Mpt(pub Rc<RefCell<State>>);

pub fn ioc(units: Vec<((u16, u16), Unit)>) -> Mpt {
    Mpt(Rc::new(RefCell::new(State {
        units: units.into_iter().collect(),
        state: READY,
        his: 0,
        mask: 0,
        phase: Phase::Idle,
        want: 0,
        msg: Vec::new(),
        reply: VecDeque::new(),
        free: VecDeque::new(),
        post: VecDeque::new(),
        log: Vec::new(),
        init: None,
        ios: Vec::new(),
        max_devices: 16,
        frame_dwords: 128,
        short_facts: false,
        refuse_init: false,
        stuck: false,
        stall: false,
        iocstatus: None,
        wrong_context: false,
        port_enable_address_reply: false,
        init_not_operational: false,
        sense_unflagged: false,
        doorbell_busy: false,
    })))
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

fn words(bytes: &[u8]) -> VecDeque<u16> {
    bytes.chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect()
}

impl State {
    fn doorbell(&mut self, v: u32) {
        if self.phase == Phase::Collecting {
            assert_eq!(self.his & IOP_DB, 0, "a word written over one not yet taken");
            self.msg.push(v);
            self.his |= IOP_DB;
            if self.msg.len() == self.want {
                self.run_handshake();
            }
            return;
        }
        match (v >> 24) as u8 {
            0x42 => {
                assert_eq!(self.phase, Phase::Idle, "a handshake over another");
                assert!(!self.doorbell_busy, "a handshake over another's");
                self.want = (v >> 16 & 0xFF) as usize;
                self.msg.clear();
                self.phase = Phase::Started { looks: 0 };
                self.his |= IOP_DB;
            }
            0x40 => {
                self.log.push(0x40);
                self.free.clear();
                self.post.clear();
                self.phase = Phase::Idle;
                self.his = 0;
                self.stall = false;
                if !self.stuck {
                    self.state = READY;
                }
            }
            f => panic!("doorbell function {f:#x}"),
        }
    }

    fn his_write(&mut self) {
        self.his &= !DB_INT;
        match self.phase {
            Phase::Started { looks } => {
                assert!(looks >= 2, "an interrupt cleared before the IOC raised it");
                self.phase = Phase::Collecting;
            }
            Phase::Replying => {
                self.reply.pop_front();
                if self.reply.is_empty() {
                    self.phase = Phase::Ending { raised: false };
                } else {
                    self.his |= DB_INT;
                }
            }
            Phase::Ending { raised: true } => self.phase = Phase::Idle,
            _ => {}
        }
    }

    fn his_read(&mut self) -> u32 {
        if let Phase::Started { looks } = self.phase {
            self.phase = Phase::Started { looks: looks.saturating_add(1) };
            if looks + 1 == 2 {
                self.his |= DB_INT;
            }
        }
        if self.phase == (Phase::Ending { raised: false }) {
            self.phase = Phase::Ending { raised: true };
            self.his |= DB_INT;
        }
        // The IOC takes a word, and shows it taken, after the host looked
        // once: a host that writes on without looking finds it still there.
        let his = self.his;
        if matches!(self.phase, Phase::Collecting | Phase::Replying) {
            self.his &= !IOP_DB;
        }
        his
    }

    fn run_handshake(&mut self) {
        let bytes: Vec<u8> = self.msg.iter().flat_map(|d| d.to_le_bytes()).collect();
        let function = bytes[3];
        self.log.push(function);
        let mut r;
        match function {
            0x03 => {
                r = vec![0u8; if self.short_facts { 0x3C } else { 0x50 }];
                r[0x1A..0x1C].copy_from_slice(&self.frame_dwords.to_le_bytes());
                r[0x24..0x26].copy_from_slice(&128u16.to_le_bytes());
                r[0x2E] = self.max_devices;
                r[0x2F] = 1;
            }
            0x02 => {
                assert_eq!(self.state, READY, "IOC INIT on an IOC not ready");
                r = vec![0u8; 20];
                self.init = Some(bytes.clone());
                if self.refuse_init {
                    r[14] = 0x01;
                } else if !self.init_not_operational {
                    self.state = OPERATIONAL;
                }
            }
            _ => {
                r = vec![0u8; 20];
                r[14] = 0x01;
            }
        }
        r[2] = (r.len() / 4) as u8;
        r[3] = function;
        r[8..12].copy_from_slice(&bytes[8..12]);
        self.reply = words(&r);
        self.phase = Phase::Replying;
        self.his |= DB_INT;
    }

    fn request(&mut self, mfa: u32) {
        assert_eq!(self.state, OPERATIONAL, "a request before IOC INIT");
        let function: u8 = rd(mfa + 3);
        let context: u32 = rd(mfa + 8);
        if function == 0x06 {
            if self.port_enable_address_reply {
                return self.address_reply(context, 0x06, 0, 0, 0, 0, 0);
            }
            return self.post.push_back(context);
        }
        assert_eq!(function, 0x00, "a SCSI IO request");
        let io = Io {
            target: rd(mfa),
            bus: rd(mfa + 1),
            cdb: (0..rd::<u8>(mfa + 4) as u32).map(|i| rd(mfa + 24 + i)).collect(),
            sense_len: rd(mfa + 5),
            context,
            lun: std::array::from_fn(|i| rd(mfa + 12 + i as u32)),
            control: rd(mfa + 20),
            data_len: rd(mfa + 40),
            sense_addr: rd(mfa + 44),
            sge_flags_len: rd(mfa + 48),
            sge_addr: rd(mfa + 52),
        };
        self.ios.push(io.clone());
        if self.stall {
            return;
        }
        let context =
            if std::mem::take(&mut self.wrong_context) { context ^ 0x10 } else { context };
        if let Some(s) = self.iocstatus.take() {
            return self.address_reply(context, 0, s, 0, 0, 0, 0);
        }
        assert_eq!(io.bus, 0);
        assert!(
            io.lun.iter().enumerate().all(|(i, &b)| i == 1 || b == 0),
            "a single-level LUN in byte 1"
        );
        let dir = io.control >> 24 & 3;
        let flags = io.sge_flags_len >> 24;
        assert_eq!(flags & 0xD1, 0xD1, "one simple SGE, last of buffer and list");
        assert_eq!(flags & 0x04 != 0, dir == 1, "host to IOC exactly for a write");
        assert_eq!(io.sge_flags_len & 0xFF_FFFF, io.data_len, "the SGE carries the data length");
        let (t, l) = (io.target as u16, io.lun[1] as u16);
        if !self.units.keys().any(|k| k.0 == t) {
            return self.address_reply(context, 0, 0x43, 0, 0, 0, 0);
        }
        let out: Vec<u8> =
            if dir == 1 { (0..io.data_len).map(|i| rd(io.sge_addr + i)).collect() } else { vec![] };
        let a = match self.units.get_mut(&(t, l)) {
            Some(u) => u.execute(&io.cdb, &out),
            None if io.cdb[0] == 0x12 => {
                let mut d = vec![0u8; 36];
                d[0] = 0x7F;
                Answer { status: 0, sense: vec![], data: d }
            }
            None => check(0x5, 0x25, 0x00),
        };
        let moved = if dir == 2 {
            let n = a.data.len().min(io.data_len as usize);
            for (i, &b) in a.data[..n].iter().enumerate() {
                wr(io.sge_addr + i as u32, b);
            }
            n as u32
        } else {
            io.data_len
        };
        for (i, &b) in a.sense.iter().take(io.sense_len as usize).enumerate() {
            wr(io.sense_addr + i as u32, b);
        }
        if a.status == 0 && moved == io.data_len {
            return self.post.push_back(context);
        }
        let status = if moved < io.data_len { 0x45 } else { 0 };
        let state = if a.sense.is_empty() || self.sense_unflagged { 0 } else { 0x01 };
        self.address_reply(context, 0, status, a.status, state, moved, a.sense.len() as u32);
    }

    #[allow(clippy::too_many_arguments)]
    fn address_reply(
        &mut self,
        context: u32,
        function: u8,
        status: u16,
        scsi: u8,
        state: u8,
        moved: u32,
        sense: u32,
    ) {
        let frame = self.free.pop_front().expect("a reply frame to answer in");
        for i in 0..32 {
            wr::<u32>(frame + 4 * i, 0);
        }
        wr::<u8>(frame + 2, 8);
        wr::<u8>(frame + 3, function);
        wr(frame + 8, context);
        wr(frame + 12, scsi);
        wr(frame + 13, state);
        wr(frame + 14, status);
        wr(frame + 20, moved);
        wr(frame + 24, sense);
        self.post.push_back(0x8000_0000 | frame >> 1);
    }
}

impl Regs for Mpt {
    fn r32(&self, off: usize) -> u32 {
        let mut s = self.0.borrow_mut();
        match off {
            0x00 => {
                let active = if s.phase == Phase::Idle && !s.doorbell_busy { 0 } else { ACTIVE };
                let word =
                    if s.phase == Phase::Replying { *s.reply.front().unwrap() as u32 } else { 0 };
                s.state | active | word
            }
            0x30 => s.his_read(),
            0x44 => s.post.pop_front().unwrap_or(0xFFFF_FFFF),
            _ => panic!("read of register {off:#x}"),
        }
    }

    fn w32(&self, off: usize, value: u32) {
        let mut s = self.0.borrow_mut();
        match off {
            0x00 => s.doorbell(value),
            0x30 => s.his_write(),
            0x34 => s.mask = value,
            0x40 => s.request(value),
            0x44 => {
                assert_eq!(value % 128, 0, "a reply frame address, frame aligned");
                s.free.push_back(value)
            }
            _ => panic!("write of register {off:#x}"),
        }
    }
}

impl Mpt {
    pub fn unit(&self, t: u16, l: u16) -> std::cell::RefMut<'_, Unit> {
        std::cell::RefMut::map(self.0.borrow_mut(), |s| s.units.get_mut(&(t, l)).expect("the unit"))
    }
}
