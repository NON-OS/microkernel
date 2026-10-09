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
//! Codecs on a link, answering the CORB from a description.
//!
//! `model::corb_engine` echoes one fixed word to every command, enough to
//! prove the ring but not a codec walk. Here a device thread reads each
//! command the driver posts, answers it the way the described codec would
//! (parameters, connection lists, pin configurations, coefficients, the jack)
//! and writes the answer into the RIRB with the answering codec's address in
//! the upper word, as a controller does. A command to an address with no
//! codec gets no answer at all, which is what the link does. Every command is
//! kept in order so a test can read back exactly what the driver sent.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::{Arc, Mutex};

use nonos_devmodel::{FakeBar, LiveDevice};

use crate::constants::{CORBWP, RIRBWP, RIRB_EX_UNSOL};
use crate::model::{live, rings, window};
use crate::regs::Regs;

pub const WCAP_VENDOR: u32 = 0xf << 20;

#[derive(Clone, Default)]
pub struct Node {
    pub params: HashMap<u16, u32>,
    pub conn: Vec<u8>,
    pub pin_cfg: u32,
}

#[derive(Clone, Default)]
pub struct SimCodec {
    pub nodes: HashMap<u8, Node>,
    pub subsystem: u32,
    /// Unsolicited responses sent ahead of the answer to every command.
    pub unsol_before_each: u32,
}

impl SimCodec {
    pub fn node(&mut self, nid: u8) -> &mut Node {
        self.nodes.entry(nid).or_default()
    }

    pub fn param(&mut self, nid: u8, p: u16, v: u32) -> &mut Self {
        self.node(nid).params.insert(p, v);
        self
    }
}

/// Codec coefficients the model holds, by codec, node and index.
pub type Coefs = Mutex<HashMap<(u8, u8, u16), u16>>;

pub struct Sim {
    pub bar: Arc<FakeBar>,
    pub corb: Arc<FakeBar>,
    pub rirb: Arc<FakeBar>,
    pub log: Arc<Mutex<Vec<u32>>>,
    pub plugged: Arc<AtomicBool>,
    pub coefs: Arc<Coefs>,
    _dev: LiveDevice,
}

impl Sim {
    pub fn regs(&self) -> Regs {
        Regs::new(self.bar.base())
    }

    pub fn link(&self) -> crate::controller::verb::Link {
        crate::controller::verb::Link::new(self.regs(), self.corb.base(), self.rirb.base(), 256)
    }

    pub fn sent(&self) -> Vec<u32> {
        self.log.lock().unwrap().clone()
    }
}

pub fn start(codecs: Vec<(u8, SimCodec)>) -> Sim {
    let bar = window();
    let (corb, rirb) = rings(0);
    let log = Arc::new(Mutex::new(Vec::new()));
    let plugged = Arc::new(AtomicBool::new(false));
    let coefs = Arc::new(Mutex::new(HashMap::new()));
    let index = Arc::new(Mutex::new(HashMap::<(u8, u8), u16>::new()));
    let consumed = Arc::new(AtomicU16::new(0));
    let produced = Arc::new(AtomicU16::new(0));
    let codecs: HashMap<u8, SimCodec> = codecs.into_iter().collect();
    let (c2, r2, l2, p2, k2) =
        (Arc::clone(&corb), Arc::clone(&rirb), Arc::clone(&log), Arc::clone(&plugged), Arc::clone(&coefs));
    let dev = live(&bar, move |b| {
        let wp = b.wrote16(CORBWP as usize) & 0xff;
        let mut c = consumed.load(Ordering::Acquire);
        while c != wp {
            c = (c + 1) & 0xff;
            let cmd = c2.wrote32(c as usize * 4);
            l2.lock().unwrap().push(cmd);
            let cad = (cmd >> 28) as u8;
            if let Some(codec) = codecs.get(&cad) {
                let put = |resp: u32, ex: u32| {
                    let r = (produced.load(Ordering::Acquire) + 1) & 0xff;
                    r2.present32(r as usize * 8, resp);
                    r2.present32(r as usize * 8 + 4, ex);
                    produced.store(r, Ordering::Release);
                    b.present16(RIRBWP as usize, r);
                };
                for _ in 0..codec.unsol_before_each {
                    put(0xdead_0000, cad as u32 | RIRB_EX_UNSOL);
                }
                let resp = answer(codec, cad, cmd, &p2, &k2, &index);
                put(resp, cad as u32);
            }
        }
        consumed.store(c, Ordering::Release);
        // Give the core back each pass: a hundred spinning device threads
        // starve one another of the milliseconds the driver's waits allow.
        std::thread::yield_now();
    });
    Sim { bar, corb, rirb, log, plugged, coefs, _dev: dev }
}

fn answer(
    codec: &SimCodec,
    cad: u8,
    cmd: u32,
    plugged: &AtomicBool,
    coefs: &Coefs,
    index: &Mutex<HashMap<(u8, u8), u16>>,
) -> u32 {
    let nid = ((cmd >> 20) & 0x7f) as u8;
    let v4 = (cmd >> 16) & 0xf;
    let node = codec.nodes.get(&nid);
    match v4 {
        0x5 => {
            index.lock().unwrap().insert((cad, nid), cmd as u16);
            return 0;
        }
        0x4 => {
            let idx = *index.lock().unwrap().get(&(cad, nid)).unwrap_or(&0);
            coefs.lock().unwrap().insert((cad, nid, idx), cmd as u16);
            return 0;
        }
        0x2 | 0x3 => return 0,
        _ => {}
    }
    let verb = (cmd >> 8) & 0xfff;
    let payload = (cmd & 0xff) as u16;
    match verb {
        0xf00 => match node {
            Some(n) => *n.params.get(&payload).unwrap_or(&if payload == 0x09 { WCAP_VENDOR } else { 0 }),
            None if payload == 0x09 => WCAP_VENDOR,
            None => 0,
        },
        0xf02 => {
            let list = node.map(|n| n.conn.clone()).unwrap_or_default();
            let mut w = 0u32;
            for k in 0..4usize {
                if let Some(&e) = list.get(payload as usize + k) {
                    w |= (e as u32) << (8 * k);
                }
            }
            w
        }
        0xf1c => node.map_or(0, |n| n.pin_cfg),
        0xf20 => codec.subsystem,
        0xf09 => {
            if plugged.load(Ordering::Acquire) {
                1 << 31
            } else {
                0
            }
        }
        0xc00 => {
            let idx = *index.lock().unwrap().get(&(cad, nid)).unwrap_or(&0);
            *coefs.lock().unwrap().get(&(cad, nid, idx)).unwrap_or(&0) as u32
        }
        _ => 0,
    }
}
