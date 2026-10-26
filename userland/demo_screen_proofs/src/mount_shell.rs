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
#[path = "shell_apps_off.rs"]
pub mod apps_off;
#[path = "../../capsule_desktop_shell/src/compositor_client/mod.rs"]
pub mod compositor_client;
#[path = "../../capsule_desktop_shell/src/frametime.rs"]
pub mod frametime;
#[path = "../../capsule_desktop_shell/src/input_router_client/mod.rs"]
pub mod input_router_client;
#[path = "../../capsule_desktop_shell/src/installer_client/mod.rs"]
pub mod installer_client;
#[path = "../../capsule_desktop_shell/src/market_client/mod.rs"]
pub mod market_client;
#[path = "../../capsule_desktop_shell/src/power_off_client/mod.rs"]
pub mod power_off_client;
#[path = "../../capsule_desktop_shell/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_desktop_shell/src/render/mod.rs"]
pub mod render;
#[path = "../../capsule_desktop_shell/src/server/mod.rs"]
pub mod server;
#[path = "shell_setup/mod.rs"]
pub mod setup;
#[path = "../../capsule_desktop_shell/src/sound/mod.rs"]
pub mod sound;
#[path = "../../capsule_desktop_shell/src/state/mod.rs"]
pub mod state;
#[path = "../../capsule_desktop_shell/src/vfs_client/mod.rs"]
pub mod vfs_client;
#[path = "../../capsule_desktop_shell/src/wait_for_setup.rs"]
pub mod wait_for_setup;
#[path = "../../capsule_desktop_shell/src/wallpaper_client/mod.rs"]
pub mod wallpaper_client;
#[path = "../../capsule_desktop_shell/src/wm_client/mod.rs"]
pub mod wm_client;
