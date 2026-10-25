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

mod claim;
mod enter;
mod exit;
mod firmware;
mod gone;
mod hold;
mod hold_acpi;
mod hold_ids;
mod hold_platform;
mod hold_rule;
mod hold_tree;
mod hold_virt;
mod leave;
mod live;
mod lookup;
mod master_off;
mod no_snoop;
mod pending;
mod port_only;
mod quiesce;
mod quiesce_all;
mod refusal;
mod release;
mod reset;
mod reset_function;
mod reset_io;
mod reset_restore;
mod reset_rule;
mod reset_run;
mod reset_wait;
mod restore;
mod restore_rule;
mod scanout;
mod scanout_rule;
mod scope_check;
mod stage_rule;
mod state;
mod types;
mod wipe;

pub use claim::claim;
pub(in crate::hardware::broker) use hold_rule::FIRMWARE_BOOT_FRAMEBUFFER;
pub(crate) use leave::retire;
pub(crate) use live::{live, while_live};
pub(crate) use lookup::{inside, lookup, InUse};
pub use quiesce_all::quiesce_all;
pub use exit::{exit_begin, exit_end};
pub use release::{quiesce_held, release};
pub use types::{Claim, ClaimError};
// Stepped at the tick that holds no lock, on every arch (L78, C-44).
pub(crate) use wipe::step_all as step_wipes;
pub(in crate::hardware::broker) use wipe::{reset_caps, say_reset_ways, ResetCaps};
