// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod cleanup;
#[cfg(target_arch = "x86_64")]
mod fb_probe;
#[cfg(target_arch = "aarch64")]
mod final_map;
mod gather;
mod handoff_init;
mod modules;
#[cfg(target_arch = "aarch64")]
mod no_tree;
// The page tables, the CR3 switch and the jump are x86_64's; aarch64 has
// its own steps, which end in arch::aarch64::exit.
#[cfg(target_arch = "x86_64")]
mod orchestrate;
#[cfg(target_arch = "aarch64")]
mod orchestrate_aarch64;
mod params;
#[cfg(target_arch = "x86_64")]
mod validate;

#[cfg(target_arch = "x86_64")]
pub use orchestrate::exit_and_jump;
#[cfg(target_arch = "aarch64")]
pub use orchestrate_aarch64::exit_and_jump;
