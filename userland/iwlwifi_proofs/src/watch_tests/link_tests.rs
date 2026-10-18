// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The link probed after a quiet access point, then ended; the firmware's
//! missed-beacons notice read.

use nonos_wifi_core::station::{PROBES, PROBE_EVERY_MS, QUIET_MS};

use super::run::after;
use crate::gen3::cmds::MAC_CONF_GROUP;
use crate::gen3::join::inbox::{Inbox, Queues};
use crate::gen3::packet::Packet;
use crate::gen3::txq::{TxQueue, TXQ_STRIDE};
use crate::join_rig::*;

#[test]
fn an_access_point_gone_quiet_is_probed_on_the_management_queue_then_the_link_ends() {
    let mut steps = vec![QUIET_MS];
    steps.extend([PROBE_EVERY_MS; PROBES as usize]);
    let (auth, sent, up, silent) = after(&rig(Security::Wpa2, Switches::default()), false, &steps);
    let probes: Vec<_> = sent.iter().filter(|s| s.1 == 0x40).collect();
    assert_eq!(probes.len(), PROBES as usize, "{sent:?}");
    assert!(!auth.is_empty() && probes.iter().all(|p| auth.contains(&p.0)), "management queue");
    assert!(!up && silent, "the link ends once every probe went unanswered");
}

#[test]
fn the_firmware_s_missed_beacons_notice_asks_at_once() {
    let (_, sent, up, _) = after(&rig(Security::Wpa2, Switches::default()), true, &[1]);
    assert!(sent.iter().any(|s| s.1 == 0x40) && up, "a probe before any quiet passed");
    let mut inbox = Inbox::new();
    let mut q = Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) };
    for (payload_len, since_rx, quiet) in [(20, 3u32, false), (20, 4, true), (6, 9, false)] {
        let mut payload = [0u8; 20];
        payload[4..8].copy_from_slice(&since_rx.to_le_bytes());
        inbox.missed = false;
        let p = Packet {
            cmd: 0xF6,
            group: MAC_CONF_GROUP,
            sequence: 0x8000,
            payload: &payload[..payload_len],
        };
        inbox.route(&p, &mut q);
        assert_eq!(inbox.missed, quiet, "{since_rx} missed, {payload_len} bytes");
    }
}
