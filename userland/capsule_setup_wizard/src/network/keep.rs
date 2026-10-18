//! Remembering the joined network when review commits.

use nonos_wifi_client::remember;

use crate::render::screens::mode;
use crate::server::say::say;
use crate::state::Context;

/// Called after the mode's Persistent flag is set, since the record is only
/// written on a boot that keeps state. The passphrase is wiped either way:
/// this is the last use setup has for it. Review commits again on the ENTER
/// after naming what was not saved; the passphrase is gone by then, and
/// remembering again would save the network with an empty one, so it does not.
pub fn keep(ctx: &mut Context) {
    let keeps = mode::keeps(ctx);
    let n = &mut ctx.net;
    if let (Some(net), true, true, true) = (n.joined, n.remember, keeps, n.pass_len > 0) {
        match remember(net.ssid(), &n.pass[..n.pass_len]) {
            Ok(gone) => {
                // The log never names the dropped network; review does.
                say(b"[SETUP] Wi-Fi network remembered, sealed with the TPM key\n");
                n.dropped = gone;
            }
            Err(e) => {
                say(b"[SETUP] Wi-Fi network not remembered: ");
                say(e.text().as_bytes());
                say(b"\n");
            }
        }
    }
    n.wipe_pass();
}
