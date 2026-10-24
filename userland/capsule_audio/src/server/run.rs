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

use nonos_libc::{mk_ipc_recv_from, mk_uptime_ms};

use super::announce::announce;
use super::pump::PumpState;
use super::pump_rule::rechoose_now;
use super::streams::StreamTable;
use super::tidy::tidy;
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
/// How often the streams of clients that ended, and a USB device come or
/// gone, are looked for.
const REAP_GAP_MS: i64 = 2_000;

pub fn run() -> ! {
    mark("[AUDIO] up\n");
    let mut mixer = Mixer::new();
    let sink = Sink::resolve();
    announce(sink.as_ref(), &mut mixer);
    let mut table = StreamTable::new();
    let mut pump = PumpState::new();
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = vec![0u8; TX_LEN];
    let mut last_reap = mk_uptime_ms();
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), RX_LEN, RECV_TIMEOUT_MS, &mut sender);
        let now = mk_uptime_ms();
        if rechoose_now(now.wrapping_sub(last_reap) >= REAP_GAP_MS, pump.take_lost()) {
            last_reap = now;
            tidy(&mut table, &mut pump, sink.as_ref());
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
