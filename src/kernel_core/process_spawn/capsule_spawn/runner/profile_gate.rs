/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The boot profile, enforced where every verified capsule starts. On an
 * Air-Gapped, Safe Mode or Recovery boot no network driver and no network
 * service ever runs, whoever asks for one, and every other program runs
 * without the Network capability: with no NIC driver and no socket service
 * there is nothing to reach, and nothing may start one later. Safe Mode also
 * starts no audio and no game. Each refusal is on the serial log by name.
 */

use crate::boot::handoff::{boot_profile, BootProfile};
use crate::capabilities::Capability;

use super::super::spec::SpawnError;

const NETWORK_DRIVERS: [&str; 6] = [
    "driver.e1000_0",
    "driver.rtl8139_0",
    "driver.rtl8169_0",
    "driver.virtio_net0",
    "driver.iwlwifi0",
    "driver.rtl8821ce0",
];
/* Safe Mode: no audio, and no game or demo beside the tools. */
const NOT_SAFE: [&str; 4] = ["driver.hda0", "audio.server", "app.snake", "app.hello"];
/* Fetches models over the network. */
const NETWORK_TOOLS: [&str; 1] = ["tool.model-fetch"];

pub(in super::super) fn check(name: &str) -> Result<(), SpawnError> {
    let profile = boot_profile();
    if refused(profile, name) {
        for part in [
            b"[PROFILE] ".as_slice(),
            profile.name().as_bytes(),
            b": not started: ",
            name.as_bytes(),
            b"\n",
        ] {
            crate::sys::serial::print(part);
        }
        return Err(SpawnError::ProfileRefused);
    }
    Ok(())
}

/* The caps a capsule runs with under this boot's profile. */
pub(in super::super) fn caps(caps: u64) -> u64 {
    if boot_profile().network() {
        caps
    } else {
        caps & !Capability::Network.bit()
    }
}

fn refused(profile: BootProfile, name: &str) -> bool {
    let network = name.starts_with("net.")
        || NETWORK_DRIVERS.contains(&name)
        || NETWORK_TOOLS.contains(&name);
    let not_safe = NOT_SAFE.iter().any(|n| is_instance(name, n));
    (network && !profile.network()) || (profile.minimal() && not_safe)
}

/* `name` is the capsule `base` or a numbered instance of it, "app.snake.1". */
fn is_instance(name: &str, base: &str) -> bool {
    name.strip_prefix(base).is_some_and(|rest| rest.is_empty() || rest.starts_with('.'))
}
