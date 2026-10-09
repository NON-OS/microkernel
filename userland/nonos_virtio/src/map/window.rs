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

//! The register window: every region a driver needs, mapped, or none.
//!
//! Each region is mapped on its own (they may sit in different BARs or far
//! apart in one) and the broker's answer is checked, not assumed: it may
//! map less than was asked when the request reaches an MSI-X table. A
//! refusal or a short mapping unmaps whatever this call already mapped, so
//! a failed attempt leaves no grant behind for the next one to trip over.

use super::plan::{plan, NOTIFY_MAP_MAX, REGION_MAP_MAX};
use crate::broker::Broker;
use crate::caps::{ModernCaps, Region};
use crate::common::NotifyArea;
use crate::error::VirtioError;
use crate::mmio::Mmio;

/// Which optional structures the driver reads. Common configuration and
/// notify are always needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Need {
    pub isr: bool,
    pub device: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Mapped {
    pub grant_id: u64,
    pub region: Mmio,
}

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub common: Mapped,
    pub notify: Mapped,
    pub notify_multiplier: u32,
    pub isr: Option<Mapped>,
    pub device: Option<Mapped>,
}

impl Window {
    /// The doorbell geometry, bounded by what is actually mapped.
    pub fn notify_area(&self) -> NotifyArea {
        NotifyArea { multiplier: self.notify_multiplier, len: self.notify.region.len() }
    }

    /// Every grant the window holds, in the order they were made.
    pub fn grant_ids(&self) -> [Option<u64>; 4] {
        [
            Some(self.common.grant_id),
            Some(self.notify.grant_id),
            self.isr.map(|m| m.grant_id),
            self.device.map(|m| m.grant_id),
        ]
    }

    /// Unmap every region, newest first. True when the broker took them all.
    pub fn unmap(&self, broker: &mut impl Broker) -> bool {
        let mut ok = true;
        for id in self.grant_ids().iter().rev().flatten() {
            ok = broker.mmio_unmap(*id) && ok;
        }
        ok
    }
}

pub fn map_window(
    broker: &mut impl Broker,
    caps: &ModernCaps,
    need: Need,
) -> Result<Window, VirtioError> {
    let common = caps.common.ok_or(VirtioError::NoCommonCfg)?;
    let notify = caps.notify.ok_or(VirtioError::NoNotifyCfg)?;
    let isr = if need.isr { Some(caps.isr.ok_or(VirtioError::NoIsrCfg)?) } else { None };
    let device =
        if need.device { Some(caps.device.ok_or(VirtioError::NoDeviceCfg)?) } else { None };
    let mut held = Held::default();
    let mapped = map_all(broker, &mut held, [common, notify], caps.notify_multiplier, isr, device);
    if mapped.is_err() {
        held.unmap(broker);
    }
    mapped
}

fn map_all(
    broker: &mut impl Broker,
    held: &mut Held,
    [common, notify]: [Region; 2],
    notify_multiplier: u32,
    isr: Option<Region>,
    device: Option<Region>,
) -> Result<Window, VirtioError> {
    let common = map_one(broker, held, common, REGION_MAP_MAX)?;
    let notify = map_one(broker, held, notify, NOTIFY_MAP_MAX)?;
    let isr = match isr {
        Some(r) => Some(map_one(broker, held, r, REGION_MAP_MAX)?),
        None => None,
    };
    let device = match device {
        Some(r) => Some(map_one(broker, held, r, REGION_MAP_MAX)?),
        None => None,
    };
    Ok(Window { common, notify, notify_multiplier, isr, device })
}

fn map_one(
    broker: &mut impl Broker,
    held: &mut Held,
    region: Region,
    cap: u32,
) -> Result<Mapped, VirtioError> {
    let plan = plan(region, cap).ok_or(VirtioError::MapRefused)?;
    let grant = broker
        .mmio_map(plan.bar, plan.map_offset, plan.map_len)
        .map_err(|_| VirtioError::MapRefused)?;
    held.push(grant.grant_id);
    let used = plan.in_page as u64 + plan.usable as u64;
    if grant.length < used {
        return Err(VirtioError::MapShort);
    }
    let base = usize::try_from(grant.user_va)
        .ok()
        .and_then(|va| va.checked_add(plan.in_page))
        .ok_or(VirtioError::MapShort)?;
    // SAFETY: the `Broker` contract keeps `grant.length` bytes at `user_va`
    // valid until the grant is unmapped, and `in_page + usable` is within
    // that length (checked above).
    let region = unsafe { Mmio::new(base as *mut u8, plan.usable) };
    Ok(Mapped { grant_id: grant.grant_id, region })
}

/// The grants made so far, for the all-or-nothing undo.
#[derive(Default)]
struct Held {
    ids: [u64; 4],
    count: usize,
}

impl Held {
    fn push(&mut self, id: u64) {
        if let Some(slot) = self.ids.get_mut(self.count) {
            *slot = id;
            self.count += 1;
        }
    }

    fn unmap(&self, broker: &mut impl Broker) {
        for id in self.ids.iter().take(self.count).rev() {
            let _ = broker.mmio_unmap(*id);
        }
    }
}
