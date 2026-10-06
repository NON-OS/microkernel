//! Joining the selected network and leaving it.

use nonos_wifi_client::ScanNetwork;

use crate::clients::compositor;
use crate::render::screens;
use crate::server::say::say;
use crate::state::Context;

/// The code the step shows when no Wi-Fi driver is running to join with.
const NO_DRIVER: i32 = -100;

/// Join `net` with what was typed. The call blocks for the whole join, so the
/// screen says so first. A failed join wipes the passphrase; a joined one
/// keeps it until review commits, in case it is to be remembered.
pub fn join(ctx: &mut Context, net: ScanNetwork) {
    let Some(driver) = ctx.net.driver else {
        ctx.net.result = Some(NO_DRIVER);
        ctx.net.wipe_pass();
        return;
    };
    ctx.net.joined = None;
    ctx.net.result = None;
    ctx.net.joining = true;
    screens::draw(ctx);
    let _ = compositor::damage_commit(ctx.compositor_port, 10, ctx.width, ctx.height);
    let r = driver.connect(net.ssid(), &ctx.net.pass[..ctx.net.pass_len]);
    ctx.net.joining = false;
    ctx.net.result = Some(r.code);
    if r.code == 0 {
        ctx.net.joined = Some(net);
        say(b"[SETUP] joined a Wi-Fi network\n");
    } else {
        ctx.net.wipe_pass();
        say(b"[SETUP] Wi-Fi join failed\n");
    }
}

/// Leave any network this step joined, and forget its passphrase.
pub fn leave(ctx: &mut Context) {
    if ctx.net.joined.take().is_some() {
        if let Some(driver) = ctx.net.driver {
            driver.disconnect();
        }
    }
    ctx.net.remember = false;
    ctx.net.result = None;
    ctx.net.wipe_pass();
}
