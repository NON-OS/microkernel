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

//! A PVSCSI adapter as QEMU's pvscsi answers the driver (hw/scsi/
//! vmw_pvscsi.c): commands with their data words, SETUP_RINGS writing the
//! ring sizes into the rings state, and each doorbell walking the request
//! ring from the adapter's consumer to the driver's producer, answering
//! each request on the completion ring from `nonos_scsi`'s simulated units.

#[path = "../../nonos_scsi/src/sim.rs"]
pub mod sim;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ptr::{read_volatile, write_volatile};
use std::rc::Rc;

use sim::{check, Answer, Unit};

use crate::regs::Regs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seen {
    /// Which doorbell carried it: true for the read/write one.
    pub rw_kick: bool,
    pub context: u64,
    pub data_addr: u64,
    pub data_len: u64,
    pub sense_addr: u64,
    pub sense_len: u32,
    pub flags: u32,
    pub cdb: Vec<u8>,
    pub lun: [u8; 8],
    pub bus: u8,
    pub target: u8,
}

pub struct State {
    pub units: BTreeMap<(u16, u16), Unit>,
    pub command: Option<u32>,
    pub words: Vec<u32>,
    pub status: u32,
    pub intr_mask: u32,
    /// Every command written, in order.
    pub commands: Vec<u32>,
    /// SETUP_RINGS's words, as last received.
    pub setup: Vec<u32>,
    pub state_at: Option<u64>,
    pub req_at: u64,
    pub cmp_at: u64,
    /// SETUP_RINGS answers failure.
    pub refuse_rings: bool,
    /// The ring sizes written are these instead of what the pages hold.
    pub wrong_log2: bool,
    /// Requests are left on the ring.
    pub stall: bool,
    /// The next request completes with this host status alone.
    pub host: Option<u16>,
    /// The next completion carries a context the driver never sent.
    pub bad_context: bool,
    pub seen: Vec<Seen>,
}

#[derive(Clone)]
pub struct Pv(pub Rc<RefCell<State>>);

pub fn adapter(units: Vec<((u16, u16), Unit)>) -> Pv {
    Pv(Rc::new(RefCell::new(State {
        units: units.into_iter().collect(),
        command: None,
        words: Vec::new(),
        status: 0,
        intr_mask: 0xFFFF_FFFF,
        commands: Vec::new(),
        setup: Vec::new(),
        state_at: None,
        req_at: 0,
        cmp_at: 0,
        refuse_rings: false,
        wrong_log2: false,
        stall: false,
        host: None,
        bad_context: false,
        seen: Vec::new(),
    })))
}

fn rd<T: Copy>(a: u64) -> T {
    // SAFETY: the rings and buffers are in the grant the test allocated,
    // whose bus address is its host address.
    unsafe { read_volatile(a as *const T) }
}

fn wr<T: Copy>(a: u64, v: T) {
    // SAFETY: as above.
    unsafe { write_volatile(a as *mut T, v) }
}

const SETUP_WORDS: usize = 132;

impl State {
    /// A reset drops whatever was on the rings; a host that stalled a
    /// request has let it go.
    fn reset(&mut self) {
        self.state_at = None;
        self.stall = false;
        self.command = None;
        self.words.clear();
    }

    fn setup_rings(&mut self) {
        self.setup = self.words.clone();
        let w = &self.words;
        let ppn = |i: usize| (w[i] as u64 | (w[i + 1] as u64) << 32) << 12;
        let (req_pages, cmp_pages) = (w[0], w[1]);
        if self.refuse_rings || !(1..=32).contains(&req_pages) || !(1..=32).contains(&cmp_pages) {
            self.status = 0xFFFF_FFFF;
            return;
        }
        assert_eq!((req_pages, cmp_pages), (1, 1), "the model holds one page per ring");
        let state = ppn(2);
        self.state_at = Some(state);
        self.req_at = ppn(4);
        self.cmp_at = ppn(68);
        for off in [0u64, 4, 12, 16] {
            wr::<u32>(state + off, 0);
        }
        let (rl, cl) = if self.wrong_log2 { (6, 7) } else { (5, 7) };
        wr::<u32>(state + 8, rl);
        wr::<u32>(state + 20, cl);
        self.status = 0;
    }

    fn kick(&mut self, rw_kick: bool) {
        let Some(st) = self.state_at else { return };
        if self.stall {
            return;
        }
        loop {
            let (prod, cons): (u32, u32) = (rd(st), rd(st + 4));
            if prod == cons {
                return;
            }
            let d = self.req_at + (cons & 31) as u64 * 128;
            let seen = Seen {
                rw_kick,
                context: rd(d),
                data_addr: rd(d + 8),
                data_len: rd(d + 16),
                sense_addr: rd(d + 24),
                sense_len: rd(d + 32),
                flags: rd(d + 36),
                cdb: (0..rd::<u8>(d + 56) as u64).map(|i| rd(d + 40 + i)).collect(),
                lun: std::array::from_fn(|i| rd(d + 57 + i as u64)),
                bus: rd(d + 66),
                target: rd(d + 67),
            };
            self.seen.push(seen.clone());
            let (host, moved, answer) = self.serve(&seen);
            let cprod: u32 = rd(st + 12);
            let c = self.cmp_at + (cprod & 127) as u64 * 32;
            let context = if std::mem::take(&mut self.bad_context) {
                seen.context ^ 0xFF
            } else {
                seen.context
            };
            wr(c, context);
            wr(c + 8, 0u64); // the emulated device never writes the data length
            wr(c + 16, answer.sense.len() as u32);
            wr(c + 20, if moved < seen.data_len && host == 0 { 0x12u16 } else { host });
            wr(c + 22, answer.status as u16);
            for (i, &b) in answer.sense.iter().take(seen.sense_len as usize).enumerate() {
                wr(seen.sense_addr + i as u64, b);
            }
            wr(st + 4, cons.wrapping_add(1));
            std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
            wr(st + 12, cprod.wrapping_add(1));
        }
    }

    /// The host status, the bytes moved and the target's answer.
    fn serve(&mut self, s: &Seen) -> (u16, u64, Answer) {
        let none = Answer { status: 0, sense: vec![], data: vec![] };
        if let Some(h) = self.host.take() {
            return (h, 0, none);
        }
        assert_eq!(s.bus, 0, "bus 0");
        assert!(
            s.lun.iter().enumerate().all(|(i, &b)| i == 1 || b == 0),
            "a single-level LUN in byte 1"
        );
        let dirs = s.flags & 0x1C;
        assert!(matches!(dirs, 0x04 | 0x08 | 0x10), "one direction flag");
        let (t, l) = (s.target as u16, s.lun[1] as u16);
        let out: Vec<u8> = if dirs == 0x10 {
            (0..s.data_len).map(|i| rd(s.data_addr + i)).collect()
        } else {
            vec![]
        };
        if dirs == 0x04 {
            assert_eq!(s.data_len, 0, "no data with DIR_NONE");
        }
        if !self.units.keys().any(|k| k.0 == t) {
            return (0x11, 0, none);
        }
        let a = match self.units.get_mut(&(t, l)) {
            Some(u) => u.execute(&s.cdb, &out),
            None if s.cdb[0] == 0x12 => {
                let mut d = vec![0u8; 36];
                d[0] = 0x7F;
                Answer { status: 0, sense: vec![], data: d }
            }
            None => check(0x5, 0x25, 0x00),
        };
        let moved = if dirs == 0x08 {
            let n = a.data.len().min(s.data_len as usize);
            for (i, &b) in a.data[..n].iter().enumerate() {
                wr(s.data_addr + i as u64, b);
            }
            n as u64
        } else if dirs == 0x10 && a.status == 0 {
            s.data_len
        } else {
            0
        };
        (0, moved, a)
    }
}

impl Regs for Pv {
    fn r32(&self, off: usize) -> u32 {
        match off {
            0x8 => self.0.borrow().status,
            _ => panic!("read of register {off:#x}"),
        }
    }

    fn w32(&self, off: usize, value: u32) {
        let mut s = self.0.borrow_mut();
        match off {
            0x0 => {
                s.commands.push(value);
                s.words.clear();
                match value {
                    1 => {
                        s.reset();
                        s.status = 0;
                    }
                    3 => s.command = Some(3),
                    _ => panic!("command {value}"),
                }
            }
            0x4 => {
                assert_eq!(s.command, Some(3), "data words with no command");
                s.words.push(value);
                if s.words.len() == SETUP_WORDS {
                    s.setup_rings();
                    s.command = None;
                }
            }
            0x2010 => s.intr_mask = value,
            0x3014 => s.kick(false),
            0x4018 => s.kick(true),
            _ => panic!("write of register {off:#x}"),
        }
    }
}

impl Pv {
    pub fn unit(&self, t: u16, l: u16) -> std::cell::RefMut<'_, Unit> {
        std::cell::RefMut::map(self.0.borrow_mut(), |s| s.units.get_mut(&(t, l)).expect("the unit"))
    }
}
