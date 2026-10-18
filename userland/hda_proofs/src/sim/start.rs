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

//! The device thread: each command the driver posts, read, logged and
//! answered into the RIRB.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::{Arc, Mutex};

use super::answer::answer;
use super::{Sim, SimCodec};
use crate::constants::{CORBWP, RIRBWP, RIRB_EX_UNSOL};
use crate::model::{live, rings, window};

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
    let (c2, r2, l2, p2, k2) = (
        Arc::clone(&corb),
        Arc::clone(&rirb),
        Arc::clone(&log),
        Arc::clone(&plugged),
        Arc::clone(&coefs),
    );
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
