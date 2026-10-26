// NONOS Operating System (AGPL-3.0-or-later)

//! The driver's reserved pages as rtw88 lays them (`rtw_build_rsvd_page`):
//! 128-byte pages, the first a one-byte stand-in beacon whose descriptor the
//! download itself supplies, then PS-Poll, QoS Null and Null, each behind a
//! descriptor of its own (`rtw_tx_rsvd_page_pkt_info_update`).

use super::frames::{null, ps_poll, qos_null};

pub const PAGE: usize = 128;
const DESC: usize = 48;
/// Page 0 holds the download's descriptor and the stand-in; the rest start
/// a page in.
const MARGIN: usize = PAGE - DESC;
/// Four pages: the buffer ends where the last descriptor and frame do.
pub const LEN: usize = 3 * PAGE + MARGIN;
pub const PS_POLL_PAGE: u8 = 1;
pub const QOS_NULL_PAGE: u8 = 2;
pub const NULL_PAGE: u8 = 3;

const W0_OFFSET: u32 = (DESC as u32) << 16;
const W0_LS: u32 = 1 << 26;
const W0_DISQSELSEQ: u32 = 1 << 31;
const W1_QSEL_MGMT: u32 = 18 << 8;
const W1_RATE_ID_B_20M: u32 = 8 << 16;
const W2_BT_NULL: u32 = 1 << 23;
const W3_USE_RATE: u32 = 1 << 8;
const W3_DISDATAFB: u32 = 1 << 10;
const W3_NAVUSEHDR: u32 = 1 << 15;
const W8_EN_HWSEQ: u32 = 1 << 15;

/// A frame's descriptor: management queue, 1 Mb/s fixed, the hardware's
/// sequence numbers except for PS-Poll, whose duration field is its AID.
fn desc(len: usize, ps_poll: bool, qos_null: bool) -> [u8; DESC] {
    let seq = if ps_poll { 0 } else { W0_DISQSELSEQ };
    let w = [
        len as u32 | W0_OFFSET | W0_LS | seq,
        W1_QSEL_MGMT | W1_RATE_ID_B_20M,
        if qos_null { W2_BT_NULL } else { 0 },
        W3_USE_RATE | W3_DISDATAFB | if ps_poll { W3_NAVUSEHDR } else { 0 },
    ];
    let mut d = [0u8; DESC];
    for (i, word) in w.iter().enumerate() {
        d[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
    if !ps_poll {
        d[32..36].copy_from_slice(&W8_EN_HWSEQ.to_le_bytes());
    }
    d
}

/// The pages for a station associated as `aid` with `bssid`.
pub fn sta_pages(aid: u16, bssid: [u8; 6], sta: [u8; 6]) -> [u8; LEN] {
    let mut b = [0u8; LEN];
    let mut put = |page: u8, frame: &[u8], ps_poll: bool, qos: bool| {
        let at = MARGIN + PAGE * (usize::from(page) - 1);
        b[at..at + DESC].copy_from_slice(&desc(frame.len(), ps_poll, qos));
        b[at + DESC..at + DESC + frame.len()].copy_from_slice(frame);
    };
    put(PS_POLL_PAGE, &ps_poll(aid, bssid, sta), true, false);
    put(QOS_NULL_PAGE, &qos_null(bssid, sta), false, true);
    put(NULL_PAGE, &null(bssid, sta), false, false);
    b
}
