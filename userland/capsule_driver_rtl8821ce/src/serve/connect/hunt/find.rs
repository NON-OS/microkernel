// NONOS Operating System (AGPL-3.0-or-later)

//! Hunting the channels for the network's beacon.

use nonos_wifi_core::dot11::parse::parse_beacon;
use nonos_wifi_core::regdom::join_channel;

use super::super::super::{SCAN_CHANNELS, SCAN_FRAME_MAX};
use super::super::probe::hunt_probe;
use super::dwell::{BEACON_HUNT_DWELL_MS, HUNT_SWEEPS};
use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;

/*
 * Hunt the channels for a beacon matching `ssid`, copying the raw frame into `out`
 * and returning its length and the channel it was heard on. The join needs the
 * real beacon (not the scan summary) because the state machine reads the BSSID,
 * channel and RSN element straight out of it.
 * Returns the matched beacon (length and channel) if found, together with the
 * number of beacons of ANY network heard during the hunt. That count turns a
 * bare "not found" into a diagnosis: zero means the radio heard nothing at all
 * on any channel (a receive-path or tuning problem), non-zero means beacons are
 * arriving but the target's was not among them (wrong SSID or it was off-air).
 */
pub(in super::super) fn find_beacon(
    link: &mut RtlLink<Regs, Grant, Grant>,
    regs: &Regs,
    ssid: &[u8],
    hidden: bool,
    our_mac: [u8; 6],
    out: &mut [u8; SCAN_FRAME_MAX],
) -> (Option<(usize, u8)>, u32) {
    let mut frame = [0u8; SCAN_FRAME_MAX];
    let mut beacons = 0u32;
    let sweeps = SCAN_CHANNELS.iter().enumerate().cycle().take(SCAN_CHANNELS.len() * HUNT_SWEEPS);
    for (i, &ch) in sweeps {
        set_rf(regs, ch, Bw::W20);
        let mut probe = [0u8; 64];
        if let Some(n) = hunt_probe(&mut probe, hidden, our_mac, ssid, ch, i) {
            link.send_raw(&probe[..n]);
        }
        let leave_at = nonos_libc::mk_uptime_ms() + BEACON_HUNT_DWELL_MS;
        while nonos_libc::mk_uptime_ms() < leave_at {
            let Some(n) = link.poll_raw(&mut frame) else {
                continue;
            };
            if let Some(info) = parse_beacon(&frame[..n]) {
                beacons += 1;
                if info.ssid == ssid {
                    // The channel the frame names, if allowed and near the
                    // one it was heard on; else where it was heard. None:
                    // not one the world domain allows, so not joined.
                    let Some(channel) = join_channel(ch, info.channel) else {
                        continue;
                    };
                    out[..n].copy_from_slice(&frame[..n]);
                    return (Some((n, channel)), beacons);
                }
            }
        }
    }
    (None, beacons)
}
