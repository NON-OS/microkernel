// NONOS Operating System (AGPL-3.0-or-later)
//! The PCI receive engine: the card writes waiting buffers into the RX and
//! RPQ rings at its own pointer, never catching up with the host's index
//! (one slot stays empty, `rx_ring_eq_is_full` false), and only while the
//! HCI's RX is on and PCIe I/O runs. The buffers are `rxbuf`'s.

pub use super::rxbuf::{wifi, wrap, TYPE_C2H, TYPE_RELEASE, TYPE_WIFI};
use super::state::State;
use super::{air, bus};

/// Before an index read: write what fits, then the word as it now stands.
pub fn fill(s: &mut State, a: u32, _v: u32) -> u32 {
    let on = s.word(0x1000) & 0x2000 != 0 && s.word(0x1010) & 0x10_0000 == 0;
    if on && !s.faults.gone {
        if a == 0x1050 {
            air::listen(s);
        }
        deliver(s, a);
    }
    s.word(a)
}

fn deliver(s: &mut State, a: u32) {
    let (desa, len) = match a {
        0x1050 => (s.word(0x1100), s.word(0x1020) & 0xffff),
        _ => (s.word(0x1108), s.word(0x1020) >> 16),
    };
    let v = s.word(a);
    let (host, mut hw) = (v & 0xfff, (v >> 16) & 0xfff);
    while len > 0 && (hw + 1) % len != host {
        let q = if a == 0x1050 { &mut s.rxq } else { &mut s.rpq };
        let Some(buf) = q.pop_front() else { break };
        let at = desa as u64 + hw as u64 * 8;
        let Some(bd) = bus::read(&s.bus, at, 8) else {
            return s.violations.push(format!("rx descriptor {at:#x} off the bus"));
        };
        let size = u16::from_le_bytes([bd[0], bd[1]]) as usize;
        let addr = u32::from_le_bytes([bd[4], bd[5], bd[6], bd[7]]) as u64;
        if buf.len() > size || !bus::write(&s.bus, addr, &buf) {
            s.violations.push(format!("rx buffer of {size} at {addr:#x} for {}", buf.len()));
        }
        hw = (hw + 1) % len;
    }
    s.regs.insert(a, host | hw << 16);
}

/// The host gave buffers back: its index must stay inside the ring.
pub fn released(s: &mut State, a: u32) {
    let len = if a == 0x1050 { s.word(0x1020) & 0xffff } else { s.word(0x1020) >> 16 };
    if s.word(a) & 0xffff >= len.max(1) {
        s.violations.push(format!("rx index {:#x} past {len}", s.word(a)));
    }
}
