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

pub mod busy;
mod claim_owned;
pub mod closing;
mod closing_rule;
mod dma_rule;
pub mod drop_attach;
pub mod exit;
mod exit_owned;
mod foreign_windows;
mod frames_of;
pub mod gate;
mod grant_detach;
pub mod grant_run;
pub mod held;
pub mod kept;
mod orphan_settle;
pub mod orphans;
mod orphans_free;
mod own_orphans;
pub(super) mod process_of;
mod reclaim_rule;
pub mod register_check;
mod register_grant;
pub mod register_race;
pub mod register_rule;
mod unmap_receiver;
pub mod unmap_rule;
mod window;
