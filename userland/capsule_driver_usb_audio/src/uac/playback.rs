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
//! The playback alternate setting a device offers, found in
//! one of its configuration descriptors (USB Audio 1.0 4.5 and 2.0 4.9, as
//! recalled, unverified).

use nonos_usbnet::desc::walk;

use super::alt::{Stream, CLASS_AUDIO, INTERFACE, SUBCLASS_STREAMING, UAC1, UAC2};
use super::pick::Alt;
use super::read::read;

/// The first alternate setting in `config` that plays 48 kHz stereo 16-bit
/// PCM through an isochronous OUT endpoint.
pub fn playback(config: &[u8]) -> Option<Stream> {
    let mut cur = Alt::default();
    for d in walk(config) {
        match (d[1], d.get(2).copied()) {
            (INTERFACE, _) if d.len() >= 9 => {
                if let Some(s) = cur.chosen() {
                    return Some(s);
                }
                let version = match d[7] {
                    UAC1 => 1,
                    UAC2 => 2,
                    _ => 0,
                };
                let streaming = d[5] == CLASS_AUDIO && d[6] == SUBCLASS_STREAMING && version != 0;
                let s = Stream { interface: d[2], alt: d[3], version, ..Stream::default() };
                cur = Alt { s, streaming, ..Alt::default() };
            }
            _ => read(&mut cur, d),
        }
    }
    cur.chosen()
}
