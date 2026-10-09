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
//! Every codec STATESTS reported, asked for its identity over the CORB, with
//! the Immediate Command interface as the fallback when the ring does not
//! reach it.
//!
//! The specification has software use one interface or the other, not both at
//! once (HDA 1.0a section 3.4.3), so the CORB is asked first and the Immediate
//! Command interface only once that send has failed; the ring is not running
//! for the codec in that case. Every present codec is probed, at whatever
//! address it answers; the analog codec is not always at 0, and the display
//! codec is usually at 2.

use crate::constants::{PARAM_VENDOR_ID, VERB_GET_PARAMETER};
use crate::controller::compose_verb;
use crate::controller::immediate;
use crate::controller::verb::Link;

pub const MAX_CODECS: usize = 15;

#[derive(Clone, Copy)]
pub struct CodecProbe {
    pub address: u8,
    pub present: u8,
    pub ok: u8,
    pub vendor_id: u16,
    pub device_id: u16,
}

pub fn probe(link: &mut Link, statests: u16) -> [CodecProbe; MAX_CODECS] {
    let mut out = [empty(); MAX_CODECS];
    let mut address = 0u8;
    while (address as usize) < MAX_CODECS {
        let present = ((statests >> address) & 1) as u8;
        out[address as usize] = if present == 0 {
            CodecProbe { address, present, ok: 0, vendor_id: 0, device_id: 0 }
        } else {
            read_vendor(link, address)
        };
        address += 1;
    }
    out
}

fn read_vendor(link: &mut Link, address: u8) -> CodecProbe {
    match link.send(compose_verb(address, 0, VERB_GET_PARAMETER, PARAM_VENDOR_ID)) {
        Ok(id) => decode(address, id),
        // The CORB did not reach this codec. The Immediate Command interface
        // is the specification's fallback for reading a codec parameter
        // without the ring (HDA 1.0a section 3.4.3); try it once.
        Err(_) => match immediate::get_parameter(link.regs(), address, PARAM_VENDOR_ID) {
            Some(id) => decode(address, id),
            None => CodecProbe { address, present: 1, ok: 0, vendor_id: 0, device_id: 0 },
        },
    }
}

fn decode(address: u8, id: u32) -> CodecProbe {
    CodecProbe {
        address,
        present: 1,
        ok: 1,
        vendor_id: (id >> 16) as u16,
        device_id: id as u16,
    }
}

const fn empty() -> CodecProbe {
    CodecProbe { address: 0, present: 0, ok: 0, vendor_id: 0, device_id: 0 }
}
