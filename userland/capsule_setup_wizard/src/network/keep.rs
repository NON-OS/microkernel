//! Remembering the joined network when review commits.

use nonos_wifi_client::remember;

use crate::render::screens::mode;
use crate::server::say::say;
use crate::state::Context;

/// Called after the mode's Persistent flag is set, since the record is only
/// written on a boot that keeps state. The passphrase is wiped either way:
/// this is the last use setup has for it.
pub fn keep(ctx: &mut Context) {
    let keeps = mode::keeps(ctx);
    let n = &mut ctx.net;
    if let (Some(net), true, true) = (n.joined, n.remember, keeps) {
        match remember(net.ssid(), &n.pass[..n.pass_len]) {
            Ok(()) => say(b"[SETUP] Wi-Fi network remembered, sealed with the TPM key\n"),
            Err(e) => {
                say(b"[SETUP] Wi-Fi network not remembered: ");
                say(e.text().as_bytes());
                say(b"\n");
            }
        }
    }
    n.wipe_pass();
}
