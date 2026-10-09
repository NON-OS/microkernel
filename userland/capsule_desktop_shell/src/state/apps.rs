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

pub const LAUNCHER_APPS: [LauncherApp; 16] = [
    LauncherApp {
        icon: LauncherIcon::Terminal,
        label: b"Terminal",
        service: b"app.terminal",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::FileManager,
        label: b"Files",
        service: b"app.file_manager",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::TextEditor,
        label: b"Editor",
        service: b"app.text_editor",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::Settings,
        label: b"Settings",
        service: b"app.settings",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::ProcessManager,
        label: b"Processes",
        service: b"app.process_manager",
        dock: true,
    },
    LauncherApp { icon: LauncherIcon::About, label: b"About", service: b"app.about", dock: true },
    LauncherApp {
        icon: LauncherIcon::Store,
        label: b"Marketplace",
        service: b"app.store",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::Calculator,
        label: b"Calculator",
        service: b"app.calculator",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::Wallet,
        label: b"Wallet",
        service: b"app.nonos_wallet",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::Browser,
        label: b"Browser",
        service: b"app.browser",
        dock: true,
    },
    /* Not a capsule: the Linux personality runs qwenchat in a window of its
     * own (apps_off::qwen). */
    LauncherApp { icon: LauncherIcon::Qwen, label: b"Qwen", service: b"tool.qwen", dock: true },
    LauncherApp {
        icon: LauncherIcon::AudioPlayer,
        label: b"Music",
        service: b"app.audio_player",
        dock: true,
    },
    LauncherApp {
        icon: LauncherIcon::VideoPlayer,
        label: b"Video",
        service: b"app.video_player",
        dock: true,
    },
    LauncherApp { icon: LauncherIcon::Snake, label: b"Snake", service: b"app.snake", dock: true },
    LauncherApp {
        icon: LauncherIcon::Install,
        label: b"Install",
        service: b"app.install",
        dock: true,
    },
    /* Opened with a picture (a desktop icon, Files, Open With); no tile. */
    LauncherApp {
        icon: LauncherIcon::ImageViewer,
        label: b"Image Viewer",
        service: b"app.image_viewer",
        dock: false,
    },
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
