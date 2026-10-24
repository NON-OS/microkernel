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

use core::cell::Cell;

use nonos_audio_proto::OUTPUT_READY;

use super::call::call;
use super::choose::SINK_LINES;
use super::lookup::{fixed, once};
use super::wire;

const USB: &[u8] = b"driver.usb_audio0";

/// The driver played through, the fixed sink chosen at start (its port and
/// its place in `SINKS`), and the USB driver. A cell: the run loop moves it
/// while the handlers hold the sink shared.
pub struct Sink {
    pub(super) port: Cell<u32>,
    pub(super) fixed: Option<(u32, usize)>,
    pub(super) usb: Option<u32>,
}

impl Sink {
    /// The fixed sink to start with. driver.usb_audio0 is spawned just
    /// before this capsule wherever an xHCI controller is, so one lookup
    /// finds it; its device is looked for later (`rechoose`), once it can
    /// have bound one.
    pub fn resolve() -> Option<Sink> {
        let usb = once(USB);
        let fixed = fixed(ready_at);
        let port = fixed.map(|(port, _)| port).or(usb)?;
        Some(Sink { port: Cell::new(port), fixed, usb })
    }

    /// The start's and each move's log line, naming the driver played through.
    pub fn output_line(&self) -> &'static str {
        match self.fixed {
            Some((port, i)) if port == self.port.get() => SINK_LINES[i],
            _ => "[AUDIO] output driver.usb_audio0\n",
        }
    }

    /// The driver's verdict on this machine's audio and its output flags,
    /// or none when it does not answer.
    pub fn output_status(&self, request_id: u32) -> Option<(u32, u32)> {
        status_at(self.port.get(), request_id)
    }
}

/// Whether the driver on `port` says this machine plays through it.
pub(super) fn ready_at(port: u32) -> bool {
    status_at(port, 0).is_some_and(|(verdict, _)| verdict == OUTPUT_READY)
}

fn status_at(port: u32, request_id: u32) -> Option<(u32, u32)> {
    let mut tx = [0u8; wire::HDR_LEN];
    let n = wire::output_status_request(request_id, &mut tx);
    let mut rx = [0u8; wire::OUTPUT_REPLY_LEN];
    let got = call(port, &tx[..n], &mut rx)?;
    wire::read_output_status(&rx[..got])
}
