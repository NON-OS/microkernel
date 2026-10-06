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

//! Finding the network before a join. The join needs the network's own
//! beacon (the MLME reads the BSSID, the RSN element and the rates out of it)
//! and the channel it was heard on, so a passive sweep runs over the NVM's
//! channels until a beacon or probe response names the network, and is then
//! stopped. Nothing is transmitted: a network that hides its name is not
//! found. A background sweep already in flight is stopped first so the hunt
//! hears every channel. Every frame heard is handed on for the scan list
//! too. The hunt is bounded by the sweep's own budget, and it never returns
//! with a sweep still in flight: the join needs the radio to itself.

use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_wifi_core::dot11::parse::parse_beacon;

use super::super::dev::Dev;
use super::super::region::{Clock, Region};
use super::super::rx_frame::RxFrame;
use super::super::sweep::{Sweep, Tick};
use crate::regs::Mmio;

/// The pause between pumps of the receive queue while hunting.
const PUMP_US: u32 = 10_000;
/// How long a stopped sweep may take to report its completion.
const ABORT_PUMPS: u32 = 100;

/// Why the hunt found nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HuntEnd {
    /// A whole sweep heard this many beacons, none of the network's.
    NotHeard(u32),
    /// The sweep could not run, or would not stop.
    Radio,
}

/// Hunt `ssid` over `channels`: its beacon and the channel it was heard on.
/// `now` reads the time the sweep's budget is held to; `heard` gets every
/// beacon and probe response.
pub fn hunt<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    sweep: &mut Sweep,
    channels: &[u8],
    ssid: &[u8],
    now: &mut dyn FnMut() -> u64,
    heard: &mut dyn FnMut(&RxFrame),
) -> Result<(Vec<u8>, u8), HuntEnd> {
    let found: RefCell<Option<(Vec<u8>, u8)>> = RefCell::new(None);
    let mut beacons = 0u32;
    let mut on_frame = |f: RxFrame| {
        heard(&f);
        let Some(b) = parse_beacon(&f.frame) else { return };
        beacons = beacons.saturating_add(1);
        if b.ssid == ssid && found.borrow().is_none() {
            *found.borrow_mut() = Some((f.frame.clone(), f.channel));
        }
    };
    stop(dev, c, sweep, &mut on_frame)?;
    match sweep.start(dev, c, channels, now(), &mut on_frame) {
        Ok(Tick::Failed) | Err(_) => return Err(HuntEnd::Radio),
        Ok(_) => {}
    }
    while sweep.running() {
        if found.borrow().is_some() {
            stop(dev, c, sweep, &mut on_frame)?;
            break;
        }
        c.delay_us(PUMP_US);
        if sweep.pump(dev, now(), &mut on_frame) == Tick::Failed {
            return Err(HuntEnd::Radio);
        }
    }
    found.into_inner().ok_or(HuntEnd::NotHeard(beacons))
}

// Stop a sweep in flight and wait, bounded, for its completion.
fn stop<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    sweep: &mut Sweep,
    on_frame: &mut dyn FnMut(RxFrame),
) -> Result<(), HuntEnd> {
    if !sweep.running() {
        return Ok(());
    }
    sweep.abort(dev, c, on_frame).map_err(|_| HuntEnd::Radio)?;
    for _ in 0..ABORT_PUMPS {
        if !sweep.running() {
            return Ok(());
        }
        c.delay_us(PUMP_US);
        // The budget is not the bound here: the completion either comes or
        // the radio is not the join's to use.
        if sweep.pump(dev, 0, on_frame) == Tick::Failed {
            return Err(HuntEnd::Radio);
        }
    }
    if sweep.running() {
        Err(HuntEnd::Radio)
    } else {
        Ok(())
    }
}
