// NONOS Operating System (AGPL-3.0-or-later)

//! The status reply's body, in named fields only: no address the radio was
//! mapped at, no BAR, no efuse content and never the factory address. The
//! two words where the BAR and the window address were stay zero, so the
//! layout readers know is kept.

use crate::efuse::EfuseDiag;
use crate::link::LinkStats;

/// The stage byte and thirteen little-endian words.
pub const STATUS_BODY_LEN: usize = 1 + 13 * 4;

/*
 * The bring-up stage, then the data-path counts: TX enqueued, TX dropped, RX
 * frames the ring returned, RX frames parsed to Ethernet, net_core requests
 * answered and RX errors. Then the last stalled efuse read (`EfuseDiag`),
 * which separates a dead register window from a live one whose efuse
 * controller is never clocked, two zero words, and what the station's
 * receive checks refused and how many group rekeys the link answered.
 */
pub fn status_body(stage: u8, s: &LinkStats, efuse: &EfuseDiag, out: &mut [u8]) -> Option<usize> {
    let out = out.get_mut(..STATUS_BODY_LEN)?;
    out[0] = stage;
    let words = [
        s.tx_ok,
        s.tx_drop,
        s.rx_ring,
        s.rx_eth,
        s.netif_reqs,
        s.rx_err,
        efuse.flags,
        efuse.stalled_at,
        efuse.bank_ldo,
        0,
        0,
        s.rx_refused,
        s.rekeys,
    ];
    for (i, w) in words.iter().enumerate() {
        out[1 + i * 4..5 + i * 4].copy_from_slice(&w.to_le_bytes());
    }
    Some(STATUS_BODY_LEN)
}
