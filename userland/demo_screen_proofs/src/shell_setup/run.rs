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

//! The shell's setup/prime/run/mod.rs, declared here with `build_context`
//! open to the crate.

#[path = "../../../capsule_desktop_shell/src/setup/prime/run/build_context.rs"]
pub(crate) mod build_context;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/commit_overlay.rs"]
mod commit_overlay;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/finish.rs"]
mod finish;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/healthcheck_peers.rs"]
mod healthcheck_peers;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/input_mask.rs"]
mod input_mask;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/register_overlay.rs"]
mod register_overlay;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/require_status.rs"]
mod require_status;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/steps.rs"]
mod steps;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/subscribe_input.rs"]
mod subscribe_input;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/subscribe_input_router.rs"]
mod subscribe_input_router;
#[path = "../../../capsule_desktop_shell/src/setup/prime/run/subscribe_wm.rs"]
mod subscribe_wm;

pub use steps::run;
pub use subscribe_input::subscribe_input;
pub use subscribe_wm::subscribe_wm;
