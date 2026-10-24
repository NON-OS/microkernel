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
// The desktop shell's modules, included at the crate root by lib.rs.
// The shell's apps_off.rs keeps its submodules in apps_off/; mounted by path
// it would look for them beside it, so shell_apps_off.rs declares them.
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "shell_apps_off.rs"]
mod apps_off;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/compositor_client/mod.rs"]
mod compositor_client;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/frametime.rs"]
mod frametime;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/input_router_client/mod.rs"]
mod input_router_client;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/installer_client/mod.rs"]
mod installer_client;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/market_client/mod.rs"]
mod market_client;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/power_off_client/mod.rs"]
mod power_off_client;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/protocol/mod.rs"]
mod protocol;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/render/mod.rs"]
pub mod render;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/server/mod.rs"]
mod server;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "shell_setup/mod.rs"]
pub mod setup;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/sound/mod.rs"]
mod sound;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/state/mod.rs"]
pub mod state;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/vfs_client/mod.rs"]
mod vfs_client;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/wait_for_setup.rs"]
mod wait_for_setup;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/wallpaper_client/mod.rs"]
mod wallpaper_client;
#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../capsule_desktop_shell/src/wm_client/mod.rs"]
mod wm_client;
