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

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_pid_alive, mk_uptime_ms};

use super::pump::PumpState;
use super::streams::StreamTable;
use super::{handle, no_sink};
use crate::mark::mark;
use crate::mixer::Mixer;
use crate::sink::Sink;

const RX_LEN: usize = 4124;
/// The longest reply the service sends.
const TX_LEN: usize = if super::proto::OUTPUT_REPLY_LEN > super::proto::VOLUME_REPLY_LEN {
    super::proto::OUTPUT_REPLY_LEN
} else {
    super::proto::VOLUME_REPLY_LEN
};
const RECV_TIMEOUT_MS: u64 = 5;
/// How often the streams of clients that ended are looked for.
const REAP_GAP_MS: i64 = 2_000;

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

pub fn run() -> ! {
    mark("[AUDIO] up\n");
    let mut mixer = Mixer::new();
    let sink = Sink::resolve();
    match sink {
        #[cfg(feature = "nonos-audio-smoketest")]
        Some(ref s) => crate::selftest::boot(&mut mixer, s),
        #[cfg(not(feature = "nonos-audio-smoketest"))]
        Some(_) => mark("[AUDIO] output driver.hda0\n"),
        /* driver.hda0 is registered before this capsule is spawned and
         * dropped when the driver finds no controller, so its absence after
         * the lookup's two seconds means this machine has no output. A
         * client asking why is told there is no sound hardware, a stream
         * open is refused with E_NODEV, everything else with E_INVAL, and
         * the desktop's chime and alerts are skipped without a sound. A
         * machine the driver found but cannot play on keeps the driver,
         * which says why (OP_OUTPUT_STATUS). */
        None => mark("[AUDIO] no output device, playback refused\n"),
    }
    let mut table = StreamTable::new();
    let mut pump = PumpState::new();
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = vec![0u8; TX_LEN];
    let mut last_reap = mk_uptime_ms();
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), RX_LEN, RECV_TIMEOUT_MS, &mut sender);
        let now = mk_uptime_ms();
        if now.wrapping_sub(last_reap) >= REAP_GAP_MS {
            last_reap = now;
            table.reap(alive);
        }
        if !nonos_libc::recv_ready(n) {
            if let Some(ref s) = sink {
                super::pump::step(&mut pump, &mut table, s);
            }
            continue;
        }
        match sink {
            Some(ref s) => {
                handle(&rx[..n as usize], &mut mixer, s, &mut table, &mut pump, &mut tx, sender)
            }
            None => no_sink(&rx[..n as usize], &mut tx),
        }
    }
}
