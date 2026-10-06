//! The lines under the network list: the driver, the join, the selected row,
//! remembering, and the wired card.

use nonos_wifi_client::{firmware_step_text, join_text, DriverStage};

use crate::network::{wired_present, NetState};
use crate::render::screens::mode;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::text::cat;
use crate::state::Context;

pub fn describe(ctx: &Context, say: &mut impl FnMut(&[u8], u32)) {
    let n = &ctx.net;
    let mut line = [0u8; 96];
    say(driver_line(n, &mut line), HINT);
    // No driver answering is not a wait: say what to do instead.
    if n.driver.is_none() && !crate::setup::machine::network_off_boot() {
        say(b"Use Ethernet or a USB Wi-Fi adapter; Settings names the chip.", HINT);
    }
    if n.joining {
        say(b"Joining. This takes up to 30 seconds.", FG);
        return;
    }
    if n.typing {
        let stars = [b'*'; 64];
        let typed = &n.pass[..n.pass_len.min(stars.len())];
        let shown: &[u8] = if n.show { typed } else { &stars[..typed.len()] };
        say(cat(&mut line, &[b"Passphrase: ", shown, b"_"]), FG);
        say(b"8 to 63 characters. TAB shows it, ESC clears it.", HINT);
    } else if let Some(code) = n.result {
        say(join_text(code).as_bytes(), if code == 0 { FG } else { HINT });
    }
    let what: &[u8] = match n.selected() {
        None => b"Setup joins no network, so nothing is sent over Wi-Fi.",
        Some(net) if net.secured => b"Secured (WPA2-Personal): type its passphrase, ENTER joins.",
        Some(_) => b"Open network: anyone nearby can read what it carries.",
    };
    say(what, FG);
    say(b"WPA3 (SAE) and enterprise networks cannot be joined.", HINT);
    if mode::keeps(ctx) {
        say(remember_line(n, &mut line), FG);
    }
    if wired_present() {
        say(b"A wired card is present: a plugged-in cable is used without asking.", HINT);
    }
}

fn driver_line<'a>(n: &NetState, line: &'a mut [u8; 96]) -> &'a [u8] {
    let Some(d) = n.driver else {
        if crate::setup::machine::network_off_boot() {
            return b"This boot runs no network: the boot menu chose it.";
        }
        // Setup holds no device list (DeviceEnum), so it cannot name the chip
        // as Settings does: either NONOS has no driver for it, or the driver
        // did not start.
        return b"No Wi-Fi driver is running: none for this chip yet, or it did not start.";
    };
    if n.stage == Some(DriverStage::FirmwareFailed) {
        if let Some(why) = n.fw_step.and_then(firmware_step_text) {
            return cat(line, &[d.label().as_bytes(), b": firmware stopped: ", why.as_bytes()]);
        }
    }
    let tail: &[u8] = match n.stage {
        Some(DriverStage::Ready) if n.count == 0 => b": listening for networks",
        Some(DriverStage::Ready) => b": ready",
        Some(stage) => return cat(line, &[d.label().as_bytes(), b": ", stage.text().as_bytes()]),
        None => b": not answering",
    };
    cat(line, &[b"Wi-Fi card: ", d.label().as_bytes(), tail])
}

fn remember_line<'a>(n: &NetState, line: &'a mut [u8; 96]) -> &'a [u8] {
    match (n.remember, n.cannot_keep) {
        (true, _) => b"[x] Remember this network (R), sealed with the TPM key",
        (false, Some(e)) => cat(line, &[b"Cannot remember it: ", e.text().as_bytes()]),
        (false, None) => b"[ ] Remember this network (R)",
    }
}
