// NONOS Operating System (AGPL-3.0-or-later)
//! A spec's descriptors on interface 1, and a lie the spec cannot say told
//! in its bytes: a descriptor cut short (its bLength with it), a rate
//! count past what Format Type I holds, or another protocol.

use super::build::Spec;
use super::encode_lie::told;
use crate::alt_sweep::lie::Cut;
use crate::devices::{format_i, iso_ep};
use crate::uac2::{ep, format2, general2};

pub fn alt(s: &Spec, alt: u8, lie: Option<(Cut, usize)>) -> Vec<u8> {
    let proto = match (lie, s.version) {
        (Some((Cut::Protocol, n)), _) => [0x10, 0x30, 0xFF][n % 3],
        (_, 1) => 0,
        _ => 0x20,
    };
    let eps = s.out.is_some() as u8 + s.inp.is_some() as u8;
    let mut d = vec![vec![9, 0x04, 1, alt, eps, 0x01, 0x02, proto, 0]];
    if let Some((Cut::Class, n)) = lie {
        (d[0][5], d[0][6]) = [(0xFF, 0x02), (0x01, 0x01), (0x01, 0x03)][n % 3];
    }
    let fmt_tag = if s.pcm { 1 } else { 2 };
    d.push(match s.version {
        1 => vec![7, 0x24, 0x01, s.link, 1, fmt_tag, 0],
        _ => {
            let mut g = general2(s.link, fmt_tag as u32, s.channels);
            g[5] = s.fmt_type;
            g
        }
    });
    d.push(match s.version {
        1 => {
            let mut f = format_i(s.channels, s.bits, &s.rates, s.range);
            f[5] = s.subframe;
            f
        }
        _ => format2(s.subframe, s.bits),
    });
    let e = |(a, attrs, mps, i): (u8, u8, u16, u8)| {
        if s.version == 1 {
            iso_ep(a, attrs, mps, i)
        } else {
            ep(a, attrs, mps, i)
        }
    };
    d.extend(s.out.map(e));
    let cs = if s.version == 1 {
        vec![7, 0x25, 0x01, s.rate_control as u8, 0, 0, 0]
    } else {
        vec![8, 0x25, 0x01, s.cs_bit as u8, 0, 0, 0, 0]
    };
    d.push(cs);
    d.extend(s.inp.map(e));
    told(&mut d, s, lie);
    if s.swap {
        d.swap(1, 2);
    }
    d.concat()
}
