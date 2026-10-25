// NONOS Operating System (AGPL-3.0-or-later)

//! Hunting the network's access points over the channels.

use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::dot11::parse::parse_beacon;
use nonos_wifi_core::regdom::join_channel;
use nonos_wifi_core::scan_list::signal_percent;

use super::super::super::dev::Dev;
use super::super::super::region::{Clock, Region};
use super::super::super::rx_frame::RxFrame;
use super::super::super::sweep::{Sweep, Tick};
use super::kinds::{HuntEnd, PUMP_US};
use super::stop::stop;
use crate::regs::Mmio;

/// Hunt over `channels` for the network `found` is for: one whole sweep,
/// every beacon and probe response for it judged into `found`, each at the
/// signal it was heard at. `now` reads the time the sweep's budget is held
/// to; `heard` gets every beacon and probe response.
pub fn hunt<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    sweep: &mut Sweep,
    channels: &[u8],
    found: &mut Candidates,
    now: &mut dyn FnMut() -> u64,
    heard: &mut dyn FnMut(&RxFrame),
) -> Result<(), HuntEnd> {
    let mut beacons = 0u32;
    let mut on_frame = |f: RxFrame| {
        heard(&f);
        let Some(b) = parse_beacon(&f.frame) else { return };
        beacons = beacons.saturating_add(1);
        // The channel the frame names if allowed and near where it was
        // heard, else where it was heard; a frame heard where the world
        // domain allows nothing is not taken.
        let Some(ch) = join_channel(f.channel, b.channel) else { return };
        found.hear(&f.frame, ch, signal_percent(f.signal));
    };
    stop(dev, c, sweep, &mut on_frame)?;
    match sweep.start(dev, c, channels, now(), &mut on_frame) {
        Ok(Tick::Failed) | Err(_) => return Err(HuntEnd::Radio),
        Ok(_) => {}
    }
    while sweep.running() {
        c.delay_us(PUMP_US);
        if sweep.pump(dev, now(), &mut on_frame) == Tick::Failed {
            return Err(HuntEnd::Radio);
        }
    }
    if found.found() {
        Ok(())
    } else {
        Err(HuntEnd::NotHeard(beacons))
    }
}
