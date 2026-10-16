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

//! One command answered: the 4-bit verbs that set a coefficient or an amp
//! here, every other verb by `answer_get`.

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use super::get_verbs::answer_get;
use super::{Coefs, SimCodec};

pub(super) fn answer(
    codec: &SimCodec,
    cad: u8,
    cmd: u32,
    plugged: &AtomicBool,
    coefs: &Coefs,
    index: &Mutex<HashMap<(u8, u8), u16>>,
) -> u32 {
    let nid = ((cmd >> 20) & 0x7f) as u8;
    let v4 = (cmd >> 16) & 0xf;
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
    answer_get(codec, cad, nid, cmd, plugged, coefs, index)
}
