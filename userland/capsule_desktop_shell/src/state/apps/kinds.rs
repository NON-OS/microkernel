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

//! An app the desktop launches: its icon, its name, its service, and
//! whether it has a dock tile.

#[derive(Clone, Copy)]
pub enum LauncherIcon {
    Terminal,
    FileManager,
    TextEditor,
    Settings,
    ProcessManager,
    About,
    Calculator,
    Snake,
    Store,
    Wallet,
    Browser,
    ImageViewer,
    AudioPlayer,
    VideoPlayer,
    Install,
    Qwen,
}

pub struct LauncherApp {
    pub icon: LauncherIcon,
    pub label: &'static [u8],
    pub service: &'static [u8],
    /// Whether the app has a tile on the dock. An app that only opens a file
    /// handed to it (the image viewer) is launched by opening one, and is
    /// still listed in the Launchpad.
    pub dock: bool,
}

/// An app with a dock tile.
pub(super) const fn tile(
    icon: LauncherIcon,
    label: &'static [u8],
    service: &'static [u8],
) -> LauncherApp {
    LauncherApp { icon, label, service, dock: true }
}

/// An app opened with a file handed to it, listed in the Launchpad only.
pub(super) const fn opened_with(
    icon: LauncherIcon,
    label: &'static [u8],
    service: &'static [u8],
) -> LauncherApp {
    LauncherApp { icon, label, service, dock: false }
}
