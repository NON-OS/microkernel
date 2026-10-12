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

//! Legacy or modern, decided once per bring-up: `modern_caps` is the
//! drivers' rule. Every function's capabilities are read, as
//! only the modern transport takes ACCESS_PLATFORM. One presenting each
//! structure its driver maps (`Need`) is driven modern; else a transitional
//! function keeps its legacy registers and a modern-only one, having none,
//! is refused by name. `wants_probe` and `choose` are virtio-blk's older rule.

use crate::broker::Broker;
use crate::caps::{parse, ModernCaps};
use crate::error::VirtioError::{self, NoCommonCfg, NoDeviceCfg, NoIsrCfg, NoNotifyCfg};
use crate::map::Need;
use crate::pci::{has_io_bar, is_modern_id, Bars, ConfigSpace};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Legacy,
    Modern,
}

/// Whether the capabilities are worth reading at all.
pub fn wants_probe(device: u16, bars: &Bars) -> bool {
    is_modern_id(device) || !has_io_bar(bars)
}

/// The transport for a function whose capabilities parsed to `caps`.
pub fn choose(device: u16, bars: &Bars, caps: &ModernCaps) -> Kind {
    if wants_probe(device, bars) && caps.common.is_some() {
        Kind::Modern
    } else {
        Kind::Legacy
    }
}

/// The capabilities to drive the function with; `None` leaves it legacy.
pub fn modern_caps(
    bars: &Bars,
    broker: &mut impl Broker,
    need: Need,
) -> Result<Option<ModernCaps>, VirtioError> {
    let cfg = ConfigSpace::read(broker).ok_or(VirtioError::ConfigRead)?;
    let caps = parse(&cfg, bars);
    match lacking(&caps, need) {
        None => Ok(Some(caps)),
        Some(missing) if cfg.u16_at(2).is_some_and(is_modern_id) => Err(missing),
        Some(_) => Ok(None),
    }
}

/// The first structure the driver maps that the function does not present.
fn lacking(caps: &ModernCaps, need: Need) -> Option<VirtioError> {
    let presented = [
        (caps.common.is_some(), NoCommonCfg),
        (caps.notify.is_some(), NoNotifyCfg),
        (caps.device.is_some() || !need.device, NoDeviceCfg),
        (caps.isr.is_some() || !need.isr, NoIsrCfg),
    ];
    presented.iter().find(|(present, _)| !present).map(|&(_, missing)| missing)
}
