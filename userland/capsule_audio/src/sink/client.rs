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
use super::choose::{choose, machine, SINKS};
use super::lookup::{once, resolve};
use super::wire;

const USB: &[u8] = b"driver.usb_audio0";

/// The driver played through, and the ones it can be: the machine's own
/// (its port and its place in `SINKS`) and the USB driver. A cell: the run
/// loop moves it while the handlers hold the sink shared.
pub struct Sink {
    pub(super) port: Cell<u32>,
    pub(super) own: Option<(u32, usize)>,
    pub(super) usb: Option<u32>,
}

impl Sink {
    /// The machine's own sink to start with, HD Audio or AC'97. The USB
    /// driver is spawned just before this capsule wherever an xHCI
    /// controller is, so one lookup finds it; its device is looked for later
    /// (`rechoose`), once it can have bound one.
    pub fn resolve() -> Option<Sink> {
        let usb = once(USB);
        let own = resolve(own_sink);
        let port = choose(false, own.map(|(port, _)| port), usb)?;
        Some(Sink { port: Cell::new(port), own, usb })
    }

    /// The driver's verdict on this machine's audio and its output flags,
    /// or none when it does not answer.
    pub fn output_status(&self, request_id: u32) -> Option<(u32, u32)> {
        status_at(self.port.get(), request_id)
    }
}

/// The machine's own sink, if one is registered now: one is taken as it
/// is, and with both the first whose status says it plays (`machine`).
fn own_sink() -> Option<(u32, usize)> {
    let ports = SINKS.map(|name| once(name.as_bytes()));
    let plays = |i: usize| {
        let verdict = ports[i].and_then(|port| status_at(port, 0));
        matches!(verdict, Some((OUTPUT_READY, _)))
    };
    let i = machine(ports.map(|port| port.is_some()), plays)?;
    Some((ports[i]?, i))
}

/// The verdict of the driver on `port`, or none when it does not answer.
pub(super) fn status_at(port: u32, request_id: u32) -> Option<(u32, u32)> {
    let mut tx = [0u8; wire::HDR_LEN];
    let n = wire::output_status_request(request_id, &mut tx);
    let mut rx = [0u8; wire::OUTPUT_REPLY_LEN];
    let got = call(port, &tx[..n], &mut rx)?;
    wire::read_output_status(&rx[..got])
}
