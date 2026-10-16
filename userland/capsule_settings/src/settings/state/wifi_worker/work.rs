// NONOS Operating System (AGPL-3.0-or-later)

//! The worker's body: one call to the driver.

use alloc::boxed::Box;

use nonos_wifi_client::remember;

use super::super::state::WIFI_NET_MAX;
use super::ask::{Answer, Ask};
use super::seat::ANSWER;
use crate::wifi::ScanNetwork;

pub(super) fn work(arg: usize) {
    // SAFETY: `arg` is the box `send` leaked for this worker, and the spawn
    // succeeded, so the worker owns it from here; dropping it wipes the
    // passphrase.
    let ask = unsafe { Box::from_raw(arg as *mut Ask) };
    let answer = perform(&ask);
    drop(ask);
    ANSWER.put(answer);
}

/// Ask the driver, on whichever thread calls this.
pub(in super::super) fn perform(ask: &Ask) -> Answer {
    match ask {
        Ask::Scan(driver) => {
            let mut nets = [ScanNetwork::EMPTY; WIFI_NET_MAX];
            let (count, outcome, stats) = driver.scan(&mut nets);
            Answer::Scanned { nets, count, outcome, stats }
        }
        Ask::Join { driver, net, pass, remember: keep } => {
            let result = driver.connect(net.ssid(), pass.as_slice());
            let kept =
                (result.code == 0 && *keep).then(|| match remember(net.ssid(), pass.as_slice()) {
                    Ok(()) => "Remembered, sealed with this machine's TPM key",
                    Err(e) => e.text(),
                });
            Answer::Joined { result, kept }
        }
    }
}
