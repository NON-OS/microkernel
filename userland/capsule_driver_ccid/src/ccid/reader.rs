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
//! Finding a CCID reader in a configuration (USB CCID 1.1, 4 and 5.1, as
//! recalled, unverified): an interface of class 0Bh with a bulk pair, its
//! 54-byte class descriptor (type 21h) after it, and an interrupt IN for
//! slot changes. Only APDU-level readers are taken (dwFeatures 20000h
//! short, 40000h short and extended): their APDUs pass through whole, where
//! TPDU and character readers would need T=0 and T=1 done here.

use nonos_usbnet::desc::{interfaces, notify_ep};
use nonos_usbnet::{Notify, Pipes};

use super::class_desc::class_descriptor;

const CLASS_SMART_CARD: u8 = 0x0B;
const SHORT_APDU: u32 = 0x0002_0000;
const EXTENDED_APDU: u32 = 0x0004_0000;
/// The most a message carries: a short APDU's 261 and its header fit far
/// inside one transfer; more is cut to that.
pub const MESSAGE_MAX: usize = 4096;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reader {
    pub interface: u8,
    pub pipes: Pipes,
    pub notify: Option<Notify>,
    /// Slots 0 to this.
    pub max_slot: u8,
    pub features: u32,
    /// dwMaxCCIDMessageLength, at most MESSAGE_MAX.
    pub max_message: usize,
}

/// Why a smart-card interface is not taken, or the reader.
pub fn find_reader(raw: &[u8]) -> Option<Result<Reader, &'static str>> {
    let ifs = interfaces(raw);
    let i = ifs.iter().find(|i| i.class == CLASS_SMART_CARD && i.alt == 0)?;
    if !i.has_bulk_pair() {
        return Some(Err("no bulk pair"));
    }
    let Some(d) = class_descriptor(raw, i.number) else { return Some(Err("no CCID descriptor")) };
    let word = |at: usize| u32::from_le_bytes([d[at], d[at + 1], d[at + 2], d[at + 3]]);
    let (features, max) = (word(40), word(44) as usize);
    if features & (SHORT_APDU | EXTENDED_APDU) == 0 {
        return Some(Err("TPDU or character level reader"));
    }
    let max_message = max.min(MESSAGE_MAX);
    let r = Reader {
        interface: i.number,
        pipes: i.pipes,
        notify: notify_ep(raw, i.number),
        max_slot: d[4],
        features,
        max_message,
    };
    Some(Ok(r))
}
