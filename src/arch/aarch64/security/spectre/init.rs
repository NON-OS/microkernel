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

use spin::Once;

use super::barrier::speculative_barrier;
use super::decide::decide;
use super::entry::publish;
use super::entry_word::word;
use super::plan::{Meltdown, Plan, Ssbd};
use super::read::{firmware, ids};
use super::{report, ssbs};
use crate::arch::aarch64::psci::{conduit, PsciMethod};
use crate::arch::aarch64::smccc::{call, version, ARCH_WORKAROUND_2};

/// The boot CPU's plan, which each later CPU compares its own against.
static BOOT_PLAN: Once<Plan> = Once::new();

/// Decide for this CPU and act on it: SSBD now, once, since it is a state
/// of the core; the entry work by folding this CPU's needs into the word
/// the EL0 vectors read. Runs on every CPU from `security::init_all`,
/// after the device tree has named the conduit and before EL0 runs.
pub fn init_spectre_mitigations() {
    let ids = ids();
    let plan = decide(&ids, &firmware());
    let via = conduit();
    match (plan.ssbd, via) {
        (Ssbd::Ssbs, _) => ssbs::clear(),
        // x1 = 1 asks for the mitigation on. What it returns says nothing
        // more than ARCH_FEATURES already did.
        (Ssbd::Firmware, Some(via)) => {
            let _ = call(via, ARCH_WORKAROUND_2, [1, 0, 0]);
        }
        _ => {}
    }
    publish(word(&plan, via == Some(PsciMethod::Hvc)));
    if plan.meltdown == Meltdown::Exposed {
        crate::arch::aarch64::security::epd1::exposed_here();
    }
    // The boot CPU runs this alone, before any other is started, so it
    // finds no plan, fills it, and always reports.
    let boot = BOOT_PLAN.get().copied();
    BOOT_PLAN.call_once(|| plan);
    if boot != Some(plan) {
        report::line(&plan, version(), ids.part());
    }
    speculative_barrier();
}

/// Whether the boot CPU's plan left the kernel exposed to Meltdown.
pub(crate) fn boot_exposed() -> bool {
    BOOT_PLAN.get().is_some_and(|plan| plan.meltdown == Meltdown::Exposed)
}
