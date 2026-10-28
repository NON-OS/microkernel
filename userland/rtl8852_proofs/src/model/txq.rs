// NONOS Operating System (AGPL-3.0-or-later)
//! The PCI transmit engine: for each channel kicked, the card walks
//! descriptors from its read pointer up to the host's write index and moves
//! its pointer on; not while the HCI's TX is off, PCIe I/O is stopped or the
//! channel's stop bit set; the WD-page channels also wait on the WP DMA.
//! Read pointer resets come from `R_AX_TXBD_RWPTR_CLR1` and `..._RXBD_...`.

use super::chans::{Ch, ACH0, CH12, CH8};
use super::state::State;
use super::{bus, frames};

/// A frame the chip put on the air: the channel, the WD's twelve words, and
/// the MPDU.
#[derive(Clone, Debug)]
pub struct Sent {
    pub ch: u8,
    pub wd: [u32; 12],
    pub frame: Vec<u8>,
    /// What went on the air: `None` when the chip had no key to seal it.
    pub air: Option<Vec<u8>>,
}

fn runs(s: &State, c: Ch) -> bool {
    let (cfg, stop) = (s.word(0x1000), s.word(0x1010));
    let wp = c.id == 12 || stop & 0x8_0000 == 0;
    let stalled = if c.id == 12 { s.faults.h2c_stall } else { s.faults.tx_stall };
    cfg & 0x800 != 0 && stop & 0x10_0000 == 0 && stop & c.stop == 0 && wp && !stalled
}

/// Walk every channel the engine may run.
pub fn pump(s: &mut State) {
    for c in [CH12, CH8, ACH0] {
        if runs(s, c) {
            walk(s, c);
        }
    }
    reports_out(s);
}

fn walk(s: &mut State, c: Ch) {
    let (v, len) = (s.word(c.idx), s.word(c.num) & 0xffff);
    let (host, mut hw) = (v & 0xfff, (v >> 16) & 0xfff);
    if len == 0 || host >= len {
        return s.violations.push(format!("ch{} index {host} of {len}", c.id));
    }
    while hw != host {
        let base = s.word(c.desa) as u64 + hw as u64 * 8;
        let Some(bd) = bus::read(&s.bus, base, 8) else {
            return s.violations.push(format!("ch{} descriptor off the bus", c.id));
        };
        let n = u16::from_le_bytes([bd[0], bd[1]]) as usize;
        let addr = u32::from_le_bytes([bd[4], bd[5], bd[6], bd[7]]) as u64;
        let data = bus::read(&s.bus, addr, n);
        // A security CAM command (category 1, class 0xa, function 1) held.
        if c.id == 12 && s.faults.sec_stall && data.as_ref().is_some_and(|d| d[24..26] == [0x29, 1])
        {
            break;
        }
        match data {
            Some(data) if c.id == 12 => frames::fwcmd(s, &data),
            Some(data) => frames::wd_page(s, c, &data),
            None => s.violations.push(format!("ch{} buffer off the bus", c.id)),
        }
        hw = (hw + 1) % len;
    }
    s.regs.insert(c.idx, host | hw << 16);
}

/// The reports gathered by one pass, in one RPQ buffer.
pub fn reports_out(s: &mut State) {
    if !s.rpp.is_empty() {
        let body: Vec<u8> = s.rpp.drain(..).flat_map(u32::to_le_bytes).collect();
        s.rpq.push_back(super::rxbuf::wrap(super::rxbuf::TYPE_RELEASE, &body));
    }
}
