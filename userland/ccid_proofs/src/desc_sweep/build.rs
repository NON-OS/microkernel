// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded reader configurations (CCID 1.1, 5.1; USB 2.0, 9.6.5, as
//! recalled, unverified): sometimes a HID interface first, whose own HID
//! descriptor is type 21h too; the reader's interface, its 54-byte class
//! descriptor before or after the bulk pair, and an interrupt IN; at
//! times an alternate setting after, with a class descriptor of its own.

use crate::exchange_sweep::rng::Rng;
use crate::reader::{class, ep};

pub struct Built {
    pub hid: Option<Vec<u8>>,
    pub iface: Vec<u8>,
    pub class: Vec<u8>,
    pub after: bool,
    pub eps: Vec<Vec<u8>>,
    pub alt: Option<Vec<u8>>,
    /// Another type's descriptor before the class one, and the alternate
    /// setting, with no endpoints, before alternate 0.
    pub vendor: Vec<u8>,
    pub alt_first: bool,
}

pub fn sound(r: &mut Rng) -> Built {
    let features =
        [0x0002_0000, 0x0004_0000, 0x0006_0000][r.range(0, 2)] | r.range(0, 0xFFFF) as u32;
    let some = r.range(0, u32::MAX as usize) as u32;
    let max = [271, 4096, 4097, some, u32::MAX][r.range(0, 4)];
    let n = r.range(0, 1) as u8;
    let mut eps = vec![ep(0x81, 0x02, 64, 0), ep(0x02, 0x02, 64, 0)];
    if r.range(0, 1) == 0 {
        eps.push(ep(0x83, 0x03, 8, 16));
    }
    let hid = (n == 1).then(|| [9, 0x21, 0x11, 0x01, 0, 1, 0x22, 0x40, 0].to_vec());
    let alt = (r.range(0, 2) == 0).then(|| class(9, 0x0004_0000, 0x1234));
    let iface = vec![9, 0x04, n, 0, eps.len() as u8, 0x0B, 0, 0, 0];
    let after = r.range(0, 1) == 0;
    let class = class(r.range(0, 255) as u8, features, max);
    Built { hid, iface, class, after, eps, alt, vendor: vec![], alt_first: false }
}

/// The configuration the parts make, a HID interface 0 when there is one.
pub fn raw(b: &Built) -> Vec<u8> {
    let mut body = vec![];
    if let Some(h) = &b.hid {
        body.extend([9, 0x04, 0, 0, 1, 0x03, 0, 0, 0]);
        body.extend(h);
        body.extend(ep(0x84, 0x03, 8, 10));
    }
    let alt = [9, 0x04, b.iface[2], 1, 0, 0x0B, 0, 0, 0];
    let tail = b.alt.as_ref().map(|a| [&alt[..], a].concat());
    if b.alt_first {
        body.extend(tail.clone().unwrap_or(alt.to_vec()));
    }
    body.extend(&b.iface);
    let (class, eps) = ([&b.vendor[..], &b.class].concat(), b.eps.concat());
    body.extend(if b.after { [eps, class].concat() } else { [class, eps].concat() });
    if !b.alt_first {
        body.extend(tail.unwrap_or_default());
    }
    let total = (9 + body.len()) as u16;
    let mut c =
        vec![9, 0x02, total as u8, (total >> 8) as u8, 1 + b.hid.is_some() as u8, 1, 0, 0x80, 50];
    c.extend(body);
    c
}
