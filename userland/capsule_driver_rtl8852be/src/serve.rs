// NONOS Operating System (AGPL-3.0-or-later)

//! Serving: one request answered per pass, then the station's work between
//! requests. net_core's link operations are served to net.core's owner
//! alone (`nonos_wifi_core::netif::gate`): the link puts raw frames on the
//! air. Everything else (the Wi-Fi control family, the status op that
//! carries the stage) goes to the shared station. A bring-up that stopped
//! still serves, so the panel can read how far it got.

use crate::radio::Station;
use crate::stage::Stage;
use crate::start::Up;
use crate::station::CapsuleEnv;
use crate::status;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_service_lookup};
use nonos_wifi_core::netif::{is_link, link_admits, refuse_link, wire, MAX_RESPONSE};
use nonos_wifi_core::softmac::SoftStation;

/// The driver's own service inbox.
const SERVICE_INBOX: u64 = 0;
/// How long a pass waits for a request before the station's idle work.
const IDLE_TIMEOUT_MS: u64 = 100;
const NET_CORE: &[u8] = b"net.core";

/// Which process owns net.core now, asked per link request so a restarted
/// net.core is followed.
fn net_core_pid() -> Option<u32> {
    let mut pid = 0u32;
    let found =
        mk_service_lookup(NET_CORE.as_ptr(), NET_CORE.len(), core::ptr::null_mut(), &mut pid);
    (found >= 0 && pid != 0).then_some(pid)
}

pub fn run(up: Option<Up>, stage: Stage) -> ! {
    let radio = up.map(Station::new);
    let mut sta = SoftStation::new(radio, CapsuleEnv, stage as u8);
    status::line(b"[rtl8852be] serving\n");
    let mut rx = [0u8; wire::HDR_LEN + wire::MAX_FRAME];
    let mut tx = [0u8; MAX_RESPONSE];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            IDLE_TIMEOUT_MS,
            &mut sender,
        );
        if n > 0 && sender != 0 {
            let req = &rx[..(n as usize).min(rx.len())];
            let len = if is_link(req) && !link_admits(sender, net_core_pid()) {
                refuse_link(req, &mut tx)
            } else {
                sta.handle(req, &mut tx)
            };
            let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
        }
        sta.tick();
    }
}
