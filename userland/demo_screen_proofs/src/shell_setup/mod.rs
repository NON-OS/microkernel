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

//! The desktop shell's setup/mod.rs, its declarations made here so the frame
//! a test paints is built by the shell's own constructor (`prime::run`
//! opened to the crate); every file is the shell's, mounted by path.

#[path = "../../../capsule_desktop_shell/src/setup/discover/mod.rs"]
mod discover;
#[path = "prime.rs"]
pub(crate) mod prime;

pub use discover::try_wallpaper;
pub(crate) use prime::wallpaper_policy;
pub use prime::{run, subscribe_input, subscribe_wm};
