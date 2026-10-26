// NONOS Operating System (AGPL-3.0-or-later)

//! Hunting the channels for the network's beacon.

use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::dot11::parse::parse_beacon;
use nonos_wifi_core::regdom::join_channel;
use nonos_wifi_core::scan_list::signal_percent;

use super::super::super::{SCAN_CHANNELS, SCAN_FRAME_MAX};
use super::super::pace::nap;
use super::super::probe::hunt_probe;
use super::dwell::{BEACON_HUNT_DWELL_MS, HUNT_SWEEPS};
use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;

/*
 * Hunt the channels for the network named `ssid`, every beacon (or, hidden,
 * probe response) for it judged into `found`. `probe_as` is set only for a
 * hidden network: the hunt's one-off address its probes go out under. The
 * hunt ends with the sweep in which a candidate was first heard, so every
 * access point of the network on any channel is heard before one is picked.
 * Returns the number of beacons of ANY network heard: zero means the radio
 * heard nothing on any channel (a receive-path or tuning problem), non-zero
 * means beacons arrive but the target's was not among them.
 * Candidates rank by security, then by the signal the card measured on the
 * frame (none, as zero), then by how often each was heard.
 */
pub(in super::super) fn find_candidates(
    link: &mut RtlLink<Regs, Grant, Grant>,
    regs: &Regs,
    ssid: &[u8],
    probe_as: Option<[u8; 6]>,
    found: &mut Candidates,
) -> u32 {
    let mut frame = [0u8; SCAN_FRAME_MAX];
    let mut beacons = 0u32;
    let sweeps = SCAN_CHANNELS.iter().enumerate().cycle().take(SCAN_CHANNELS.len() * HUNT_SWEEPS);
    for (i, &ch) in sweeps {
        if i == 0 && found.found() {
            break;
        }
        set_rf(regs, ch, Bw::W20);
        let mut probe = [0u8; 64];
        if let Some(n) = probe_as.and_then(|mac| hunt_probe(&mut probe, true, mac, ssid, ch, i)) {
            link.send_raw(&probe[..n]);
        }
        let leave_at = nonos_libc::mk_uptime_ms() + BEACON_HUNT_DWELL_MS;
        while nonos_libc::mk_uptime_ms() < leave_at {
            let Some((n, dbm)) = link.poll_heard(&mut frame) else {
                nap();
                continue;
            };
            let Some(info) = parse_beacon(&frame[..n]) else {
                continue;
            };
            beacons += 1;
            // The channel the frame names, if allowed and near the one it
            // was heard on; else where it was heard. None: not one the
            // world domain allows, so not joined.
            if let Some(channel) = join_channel(ch, info.channel) {
                found.hear(&frame[..n], channel, dbm.map_or(0, signal_percent));
            }
        }
    }
    beacons
}
