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

//! The apps the desktop launches, dock apps first, and how many have a
//! dock tile.

use super::kinds::{opened_with, tile, LauncherApp, LauncherIcon};

pub const LAUNCHER_APPS: [LauncherApp; 16] = [
    tile(LauncherIcon::Terminal, b"Terminal", b"app.terminal"),
    tile(LauncherIcon::FileManager, b"Files", b"app.file_manager"),
    tile(LauncherIcon::TextEditor, b"Editor", b"app.text_editor"),
    tile(LauncherIcon::Settings, b"Settings", b"app.settings"),
    tile(LauncherIcon::ProcessManager, b"Processes", b"app.process_manager"),
    tile(LauncherIcon::About, b"About", b"app.about"),
    tile(LauncherIcon::Store, b"Marketplace", b"app.store"),
    tile(LauncherIcon::Calculator, b"Calculator", b"app.calculator"),
    tile(LauncherIcon::Wallet, b"Wallet", b"app.nonos_wallet"),
    tile(LauncherIcon::Browser, b"Browser", b"app.browser"),
    /* Not a capsule: the Linux personality runs qwenchat in a window of its
     * own (apps_off::qwen). */
    tile(LauncherIcon::Qwen, b"Qwen", b"tool.qwen"),
    tile(LauncherIcon::AudioPlayer, b"Music", b"app.audio_player"),
    tile(LauncherIcon::VideoPlayer, b"Video", b"app.video_player"),
    tile(LauncherIcon::Snake, b"Snake", b"app.snake"),
    tile(LauncherIcon::Install, b"Install", b"app.install"),
    /* Opened with a picture (a desktop icon, Files, Open With); no tile. */
    opened_with(LauncherIcon::ImageViewer, b"Image Viewer", b"app.image_viewer"),
];

/// How many apps have a tile on the dock. The dock's apps lead the table, so
/// a dock tile's position is its app's index.
pub const DOCK_APPS: usize = {
    let mut n = 0;
    while n < LAUNCHER_APPS.len() && LAUNCHER_APPS[n].dock {
        n += 1;
    }
    n
};
