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

//! A claim whose isolation group another vendor driver holds, shared on a
//! standard boot. When all that joins the two devices is a port without
//! ACS, each still gets its own IOMMU domain, so neither reaches memory its
//! capsule was not given; only traffic between the two devices is not
//! stopped by the hardware. Linux drives such a pair so, and 0.9.2 drove
//! them unconfined: a Gemini Lake laptop's Wi-Fi beside its NVMe disk was
//! refused for it. A hardened or air-gapped boot refuses, as before, and an
//! alias or an unreadable group is refused on every boot. Said on its
//! [VT-D] line and in MkProcStat's header. kernel_proofs::group_share.

extern crate alloc;

use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

use super::acs_refusal::shared_line;
use super::group_rule::may_share;
use super::port_only::Blocked;
use super::say::say;
use crate::boot::handoff::{boot_profile, BootProfile};
use crate::security::attest_registry::authority_of;
use crate::security::dev_roots::Authority;

const BUILT: bool = cfg!(feature = "nonos-shared-group-posture");

static SHARED: AtomicBool = AtomicBool::new(false);

/// Whether `claimer`'s claim goes ahead beside `holder`, whose device
/// blocks it as `b`: said, and kept for MkProcStat, when it does.
pub(super) fn shares(claimer: u32, holder: u32, b: &Blocked) -> bool {
    let hardened = matches!(boot_profile(), BootProfile::Hardened | BootProfile::AirGapped);
    let vendor = [claimer, holder].iter().all(|&p| authority_of(p) == Some(Authority::Vendor));
    let reaches = matches!(b, Blocked::Reaches(..));
    if !may_share(BUILT, hardened, vendor, reaches) {
        return false;
    }
    if let Blocked::Reaches(at, hop) = *b {
        say(shared_line(at, hop, &holder_name(holder)).as_bytes());
    }
    SHARED.store(true, Ordering::Release);
    true
}

/// Whether any claim this boot shared its group.
pub(crate) fn held() -> bool {
    SHARED.load(Ordering::Acquire)
}

/// The holder's process name, never its pid (K10).
pub(super) fn holder_name(pid: u32) -> String {
    crate::process::with_process(pid, |p| p.name.lock().clone())
        .unwrap_or_else(|| String::from("another capsule"))
}
