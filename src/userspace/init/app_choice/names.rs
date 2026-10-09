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

/* Which switch covers a capsule, by each name it is started under. */

use super::bits::{BROWSER, CALCULATOR, EDITOR, FILES, LINUX, MEDIA, STORE, WALLET};
use super::gate::is_off;
use crate::userspace::init::PendingApp;

/* A capsule init starts at boot, by the name it registers it under. */
pub(crate) fn capsule_off(name: &str) -> bool {
    is_off(match name {
        "app_browser" => BROWSER,
        "app_nonos_wallet" => WALLET,
        "app_store" => STORE,
        "app_file_manager" => FILES,
        "app_text_editor" => EDITOR,
        "app_calculator" => CALCULATOR,
        "app_audio_player" | "video_player" | "image_viewer" => MEDIA,
        "app_linux" => LINUX,
        _ => 0,
    })
}

/* A window the dock asks for. */
pub(crate) fn window_off(app: PendingApp) -> bool {
    is_off(match app {
        PendingApp::Browser => BROWSER,
        PendingApp::WalletNonos => WALLET,
        PendingApp::FileManager => FILES,
        PendingApp::TextEditor => EDITOR,
        PendingApp::Calculator => CALCULATOR,
        PendingApp::AudioPlayer | PendingApp::VideoPlayer => MEDIA,
        _ => 0,
    })
}

/* Linux packages, installed or run: the personality runs every one. */
pub(crate) fn linux_off() -> bool {
    is_off(LINUX)
}

/* A tool a terminal runs: `linux`, Qwen and its model fetcher need the personality. */
pub(crate) fn tool_off(name: &[u8]) -> bool {
    matches!(name, b"tool.linux" | b"tool.qwen" | b"tool.model-fetch") && is_off(LINUX)
}
