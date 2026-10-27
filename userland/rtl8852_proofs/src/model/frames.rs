// NONOS Operating System (AGPL-3.0-or-later)
//! What the card does with each buffer it fetches. CH12 carries a 24-byte
//! WD body and then a firmware download chunk or an H2C command; ACH0 and
//! CH8 carry a WD page (48-byte WD, WP info, address info) naming the frame,
//! which goes on the air and is reported released on the RPQ (`txq`).

use super::chans::Ch;
use super::cpu::{self, Fwdl, H2c};
use super::state::State;
use super::{air, bus, h2c_seen, rxq};

fn w(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap())
}

pub fn fwcmd(s: &mut State, data: &[u8]) {
    let (d0, size) = (w(data, 0), (w(data, 2) & 0x3fff) as usize);
    if (d0 >> 16) & 0xf != 12 || data.len() != 24 + size {
        return s.violations.push(format!("fwcmd wd {d0:#x}, size {size} of {}", data.len()));
    }
    let p = &data[24..];
    if d0 & 0x10_0000 != 0 {
        if p.len() > 2020 {
            s.violations.push(format!("download chunk of {}", p.len()));
        }
        return cpu::section_bytes(s, p);
    }
    let (h0, h1) = (w(p, 0), w(p, 1));
    let (cat, class, func) = ((h0 & 3) as u8, ((h0 >> 2) & 0x3f) as u8, (h0 >> 8) as u8);
    if (h1 & 0x3fff) as usize != p.len() {
        return s.violations.push(format!("h2c length {} of {}", h1 & 0x3fff, p.len()));
    }
    let (rack, dack) = (h1 & (1 << 14) != 0, h1 & (1 << 15) != 0);
    let c = H2c { cat, class, func, seq: (h0 >> 24) as u8, rack, dack, body: p[8..].to_vec() };
    match s.fwdl {
        Fwdl::WaitHdr if c.is(1, 3, 0) => cpu::header(s, &c.body),
        Fwdl::Running => {
            h2c_seen::took(s, &c);
            if rack || dack {
                let ack = [cat, class, func, c.seq];
                s.rxq.push_back(rxq::wrap(rxq::TYPE_C2H, &ack));
            }
            s.h2c.push(c);
        }
        _ => s.violations.push(format!("h2c {cat}/{class}/{func} in {:?}", s.fwdl)),
    }
}

pub fn wd_page(s: &mut State, c: Ch, page: &[u8]) {
    if page.len() != 64 {
        return s.violations.push(format!("ch{} page of {}", c.id, page.len()));
    }
    let wd: [u32; 12] = core::array::from_fn(|i| w(page, i));
    let (ch, qsel, size) = ((wd[0] >> 16) & 0xf, (wd[2] >> 17) & 0x3f, wd[2] & 0x3fff);
    let want = if c.id == 8 { 0x12 } else { 0 };
    let (seq, valid) = (w(page, 12) & 0x7fff, w(page, 12) & 0x8000 != 0);
    let (len, flags, addr) = (w(page, 14) & 0xffff, w(page, 14) >> 16, w(page, 15));
    if ch != c.id as u32 || qsel != want || !valid || size != len || flags != 0x8001 {
        let why =
            format!("ch{} wd ch {ch} qsel {qsel:#x} size {size}/{len} flags {flags:#x}", c.id);
        return s.violations.push(why);
    }
    let Some(frame) = bus::read(&s.bus, addr as u64, len as usize) else {
        return s.violations.push(format!("ch{} frame off the bus", c.id));
    };
    air::transmit(s, &frame);
    s.sent.push(super::txq::Sent { ch: c.id, wd, frame });
    if !s.faults.no_release {
        s.rpp.push(seq << 16 | qsel << 8);
    }
}
