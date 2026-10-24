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

//! The desktop shell's apps_off.rs, its declarations made with each
//! submodule's own path (the file sits beside its directory, apps_off/).

#[path = "../../capsule_desktop_shell/src/apps_off/dim.rs"]
mod dim;
#[path = "../../capsule_desktop_shell/src/apps_off/expect.rs"]
mod expect;
#[path = "../../capsule_desktop_shell/src/apps_off/failed.rs"]
mod failed;
#[path = "../../capsule_desktop_shell/src/apps_off/mask.rs"]
mod mask;
#[path = "../../capsule_desktop_shell/src/apps_off/open.rs"]
mod open;
#[path = "../../capsule_desktop_shell/src/apps_off/qwen/mod.rs"]
mod qwen;
#[path = "../../capsule_desktop_shell/src/apps_off/say.rs"]
mod say;
pub use dim::dim;
pub use expect::{expect, say_default};
pub use failed::toast_failed;
pub use mask::is_off;
pub use open::{open, request};
