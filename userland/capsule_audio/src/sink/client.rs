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

use super::call::call;
use super::choose::choose;
use super::lookup::{once, resolve};
use super::wire;

const HDA: &[u8] = b"driver.hda0";
const USB: &[u8] = b"driver.usb_audio0";

/// The driver played through, and the two it can be. A cell: the run loop
/// moves it while the handlers hold the sink shared.
pub struct Sink {
    pub(super) port: Cell<u32>,
    pub(super) hda: Option<u32>,
    pub(super) usb: Option<u32>,
}

impl Sink {
    /// HD Audio to start with. driver.usb_audio0 is spawned just before this
    /// capsule wherever an xHCI controller is, so one lookup finds it; its
    /// device is looked for later (`rechoose`), once it can have bound one.
    pub fn resolve() -> Option<Sink> {
        let usb = once(USB);
        let hda = resolve(HDA);
        let port = choose(false, hda, usb)?;
        Some(Sink { port: Cell::new(port), hda, usb })
    }

    /// The driver's verdict on this machine's audio and its output flags,
    /// or none when it does not answer.
    pub fn output_status(&self, request_id: u32) -> Option<(u32, u32)> {
        self.status_at(self.port.get(), request_id)
    }

    pub(super) fn status_at(&self, port: u32, request_id: u32) -> Option<(u32, u32)> {
        let mut tx = [0u8; wire::HDR_LEN];
        let n = wire::output_status_request(request_id, &mut tx);
        let mut rx = [0u8; wire::OUTPUT_REPLY_LEN];
        let got = call(port, &tx[..n], &mut rx)?;
        wire::read_output_status(&rx[..got])
    }
}
