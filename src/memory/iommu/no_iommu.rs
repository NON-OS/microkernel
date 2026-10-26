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
//! The no-IOMMU posture in force on this boot (no_iommu_rule.rs). Decided once,
//! after the units are brought up and before any capsule can claim a device,
//! and stated on its own [IOMMU] line the moment it is taken.

use core::sync::atomic::{AtomicBool, Ordering};

use super::no_iommu_refusal::stops;
use super::no_iommu_rule::{enters, reason, refusing_boot};
use super::query::capabilities;
use super::vendor::IommuVendor;
use crate::boot::handoff::{boot_profile, BootProfile};
use crate::sys::serial::Line;

pub(crate) const BUILT: bool = cfg!(feature = "nonos-dma-unconfined-posture");
const AGENTPROOF: bool = cfg!(feature = "nonos-agentproof");

static ACTIVE: AtomicBool = AtomicBool::new(false);

/// Take the posture if the rule says so, and say which way it went when no
/// unit is in service, so a refused boot reads as plainly as a taken one.
pub fn decide(vendor: IommuVendor) {
    if capabilities().enforcing {
        return;
    }
    let profile = boot_profile();
    let (hardened, air_gapped) =
        (profile == BootProfile::Hardened, profile == BootProfile::AirGapped);
    let refusing = refusing_boot(hardened, air_gapped) || AGENTPROOF;
    let why = reason(vendor != IommuVendor::Absent);
    let taken = enters(BUILT, false, refusing);
    ACTIVE.store(taken, Ordering::Release);
    let mut line = Line::new();
    line.str(if taken { b"[IOMMU] posture=dma-unconfined" } else { b"[IOMMU] posture=refused" });
    line.str(b" reason=").str(why.name());
    line.str(b" profile=").str(profile.name().as_bytes());
    line.str(b" built=").str(if BUILT { b"1" } else { b"0" });
    line.str(if taken {
        b"; a claimed device may reach all of memory"
    } else {
        b"; every bus-master claim is refused"
    })
    .end();
    if let Some(needs) = stops(false, AGENTPROOF, hardened, air_gapped) {
        crate::boot::stop("IOMMU", needs.line());
    }
}

/// Whether the posture is in force: claims go ahead unconfined.
pub(crate) fn holds() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

pub(crate) fn posture_tail() -> &'static [u8] {
    if holds() {
        b", posture=dma-unconfined"
    } else {
        b""
    }
}
