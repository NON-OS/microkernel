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

//! The verbs with a 12-bit code, answered from the codec's description.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use super::{Coefs, SimCodec, WCAP_VENDOR};

pub(super) fn answer_get(
    codec: &SimCodec,
    cad: u8,
    nid: u8,
    cmd: u32,
    plugged: &AtomicBool,
    coefs: &Coefs,
    index: &Mutex<HashMap<(u8, u8), u16>>,
) -> u32 {
    let node = codec.nodes.get(&nid);
    let verb = (cmd >> 8) & 0xfff;
    let payload = (cmd & 0xff) as u16;
    if let Some(word) = super::noise::undefined(codec, nid, verb, payload) {
        return word;
    }
    match verb {
        0xf00 => match node {
            Some(n) => {
                *n.params.get(&payload).unwrap_or(&if payload == 0x09 { WCAP_VENDOR } else { 0 })
            }
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
