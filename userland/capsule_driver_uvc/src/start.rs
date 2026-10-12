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
//! A stream agreed and begun: the probe set and read back (the device
//! answers what it will do), committed, the smallest alternate with room
//! for the committed payload selected, and its endpoint configured.

use nonos_usbnet::bus::Bus;
use nonos_usbnet::setup::Setup;
use nonos_usbnet::xhci::{XhciBus, E_INVAL};
use nonos_uvc::{pick_alt, Alt, Probe, CONTROL_LEN_11, GET_CUR, SET_CUR, VS_COMMIT, VS_PROBE};

use crate::bind::Cam;
use crate::isoch::configure;

pub struct Stream {
    pub alt: Alt,
    pub max_frame: u32,
}

/// The format and frame indices are the camera's own; `interval` 0 takes
/// the frame's default.
pub fn start(
    bus: &mut XhciBus,
    c: &Cam,
    (format, frame, interval): (u8, u8, u32),
) -> Result<Stream, i32> {
    let f = c.cam.formats.iter().find(|f| f.index == format).ok_or(E_INVAL)?;
    let fr = f.frames.iter().find(|x| x.index == frame).ok_or(E_INVAL)?;
    let interval = if interval == 0 { fr.interval } else { interval };
    let want = Probe { hint: 1, format, frame, interval, max_frame: 0, max_payload: 0 };
    let vs = c.cam.streaming;
    let probe = |sel: u8| Setup::class(false, SET_CUR, (sel as u16) << 8, vs);
    bus.control_out(probe(VS_PROBE), &want.bytes())?;
    let mut got = [0u8; CONTROL_LEN_11];
    let n = bus.control_in(Setup::class(true, GET_CUR, (VS_PROBE as u16) << 8, vs), &mut got)?;
    let agreed = Probe::read(&got[..n]).ok_or(E_INVAL)?;
    bus.control_out(probe(VS_COMMIT), &agreed.bytes())?;
    let alt = pick_alt(&c.cam.alts, agreed.max_payload).ok_or(E_INVAL)?;
    bus.control_out(Setup::set_interface(vs, alt.setting), &[])?;
    configure(bus.xhci, bus.slot, &alt, alt.interval)?;
    let max_frame = if agreed.max_frame == 0 { fr.max_bytes } else { agreed.max_frame };
    Ok(Stream { alt, max_frame })
}

/// The stream stopped: alternate 0 again, so the bandwidth is given back.
pub fn stop(bus: &mut XhciBus, c: &Cam) -> Result<(), i32> {
    bus.control_out(Setup::set_interface(c.cam.streaming, 0), &[]).map(|_| ())
}
