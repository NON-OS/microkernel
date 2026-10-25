// NONOS Operating System (AGPL-3.0-or-later)
//! The stream a host takes from a configuration, stated apart from the
//! driver's find, read and format (USB Audio 1.0 4.5, Format 2.2; 2.0 4.9,
//! as recalled, unverified). Descriptors are read by their bLength until
//! one under 2 or past the end. An interface descriptor (9 bytes or more)
//! begins an alternate: a stream if of class 1, subclass 2, protocol 0
//! (1.0) or 20h (2.0). Within it: AS_GENERAL (24h/1) names the terminal,
//! and says PCM (1.0 wFormatTag 1 in 7 bytes; 2.0 Type I and bmFormats
//! bit 0, with its channel count, in 11); Format Type I (24h/2) of
//! two-byte subframes and 16 bits (1.0: 8 bytes, its rates all held,
//! 48 kHz listed or within its range, and its channel count; 2.0: 6);
//! isoch endpoints of 7 bytes or more, OUT, or IN unless feedback; a 1.0
//! class endpoint's bit 0 is its rate control. The first alternate that
//! plays (or captures) as build.rs says is taken.

use super::model_alt::{taken, Alt};
use super::model_format::format;
use crate::uac::Stream;

pub fn find(c: &[u8], play: bool) -> Option<Stream> {
    let (mut at, mut a) = (0, Alt::default());
    while at + 2 <= c.len() && c[at] >= 2 && at + c[at] as usize <= c.len() {
        let d = &c[at..at + c[at] as usize];
        at += d.len();
        let v = a.s.version;
        match (d[1], d.get(2).copied().unwrap_or(0)) {
            (0x04, _) if d.len() >= 9 => {
                if let Some(s) = taken(&a, play) {
                    return Some(s);
                }
                let version = match d[7] {
                    0 => 1,
                    0x20 => 2,
                    _ => 0,
                };
                a = Alt { stream: d[5] == 1 && d[6] == 2 && version > 0, ..Alt::default() };
                (a.s.interface, a.s.alt, a.s.version) = (d[2], d[3], version);
            }
            (0x24, 1) if d.len() >= 4 => {
                a.s.terminal = d[3];
                a.pcm = (v == 1 && d.len() >= 7 && d[5] == 1 && d[6] == 0)
                    || (v == 2 && d.len() >= 11 && d[5] == 1 && d[6] & 1 == 1);
                if v == 2 && d.len() >= 11 {
                    a.s.channels = d[10];
                }
            }
            (0x24, 2) => {
                (a.format, a.s.channels) = format(v, d, a.s.channels);
            }
            (0x05, addr) if d.len() >= 7 && d[3] & 3 == 1 => {
                let ep = Some((addr, d[3], u16::from_le_bytes([d[4], d[5]]), d[6]));
                if addr & 0x80 == 0 {
                    a.out = ep
                } else if d[3] >> 4 & 3 != 1 {
                    a.inp = ep
                }
            }
            (0x25, 1) if d.len() >= 4 => a.s.rate_control = v == 1 && d[3] & 1 == 1,
            _ => {}
        }
    }
    taken(&a, play)
}
