// NONOS Operating System (AGPL-3.0-or-later)

//! Hunting the network's beacon over the channels.

use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_wifi_core::dot11::parse::parse_beacon;

use super::super::super::dev::Dev;
use super::super::super::region::{Clock, Region};
use super::super::super::rx_frame::RxFrame;
use super::super::super::sweep::{Sweep, Tick};
use super::kinds::{HuntEnd, PUMP_US};
use super::stop::stop;
use crate::regs::Mmio;

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
