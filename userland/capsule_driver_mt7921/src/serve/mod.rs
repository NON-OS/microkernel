/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Serving `driver.mt7921e0`: every request handed to the shared station
//! (`nonos_wifi_core::softmac::SoftStation`), which answers the Wi-Fi
//! control family (status, scan, connect, disconnect, link, radio) and
//! net.core's link protocol, and steps the scan and the session between
//! requests. The link is served to net.core's owner alone
//! (`nonos_wifi_core::netif::gate`); anyone else gets EPERM. A radio that
//! did not come up still serves, its link down and its stage answered.

pub mod env;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};
use nonos_wifi_core::netif::wire::{HDR_LEN, MAX_FRAME};
use nonos_wifi_core::netif::{is_link, link_admits, refuse_link, MAX_RESPONSE};
use nonos_wifi_core::softmac::SoftStation;

use crate::radio::Mt7921;
use crate::stage::Stage;
use env::{net_core_pid, Kernel};

/// The service's own inbox.
const INBOX: u64 = 0;
/// How long a pass waits for a request before stepping the station: well
/// under the scan's dwell, so a sweep moves on whether or not anyone asks.
const IDLE_MS: u64 = 20;

/// The reply to `req` from `sender` into `out`; its length.
pub fn answer(
    st: &mut SoftStation<Mt7921, Kernel>,
    req: &[u8],
    sender: u32,
    out: &mut [u8],
) -> usize {
    if is_link(req) && !link_admits(sender, net_core_pid()) {
        return refuse_link(req, out);
    }
    st.handle(req, out)
}

pub fn run(radio: Option<Mt7921>, stage: Stage) -> ! {
    let mut st = SoftStation::new(radio, Kernel, stage as u8);
    crate::say::line("serving driver.mt7921e0");
    let mut rx = [0u8; HDR_LEN + MAX_FRAME + 64];
    let mut tx = [0u8; MAX_RESPONSE + 512];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(INBOX, rx.as_mut_ptr(), rx.len(), IDLE_MS, &mut sender);
        if n > 0 && sender != 0 {
            let len = answer(&mut st, &rx[..n as usize], sender, &mut tx);
            let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
            rx[..n as usize].fill(0);
        }
        st.tick();
    }
}
